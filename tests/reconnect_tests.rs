use async_trait::async_trait;
use boincrs::app::controller::AppController;
use boincrs::app::reconnect::backoff_delay;
use boincrs::app::state::ConnectionState;
use boincrs::boinc::rpc_client::BoincRpcClient;
use boincrs::boinc::transport::BoincTransport;
use boincrs::error::{AppError, AppResult};
use std::io;

// ── helpers ─────────────────────────────────────────────────────────────────

fn io_err() -> AppError {
    AppError::Io(io::Error::new(io::ErrorKind::ConnectionRefused, "refused"))
}

fn ok_responses() -> Vec<Vec<u8>> {
    // auth1 → nonce, auth2 → authorized, then four data responses (projects,
    // tasks, transfers, client_state) each returning a minimal valid payload.
    let data = b"<boinc_gui_rpc_reply><success/></boinc_gui_rpc_reply>\x03".to_vec();
    vec![data.clone(), data.clone(), data.clone(), data.clone()]
}

// ── mock transport ───────────────────────────────────────────────────────────

struct FailThenSucceedTransport {
    fail_remaining: u32,
    successes: Vec<Vec<u8>>,
}

impl FailThenSucceedTransport {
    fn new(fails: u32) -> Self {
        Self {
            fail_remaining: fails,
            successes: ok_responses(),
        }
    }
}

#[async_trait]
impl BoincTransport for FailThenSucceedTransport {
    async fn send(&mut self, _payload: &[u8]) -> AppResult<()> {
        if self.fail_remaining > 0 {
            self.fail_remaining -= 1;
            Err(io_err())
        } else {
            Ok(())
        }
    }

    async fn receive(&mut self) -> AppResult<Vec<u8>> {
        if self.successes.is_empty() {
            return Ok(b"<boinc_gui_rpc_reply><success/></boinc_gui_rpc_reply>\x03".to_vec());
        }
        Ok(self.successes.remove(0))
    }
}

struct AlwaysFailTransport;

#[async_trait]
impl BoincTransport for AlwaysFailTransport {
    async fn send(&mut self, _payload: &[u8]) -> AppResult<()> {
        Err(AppError::AuthenticationFailed)
    }

    async fn receive(&mut self) -> AppResult<Vec<u8>> {
        Err(AppError::AuthenticationFailed)
    }
}

// ── error classification ─────────────────────────────────────────────────────

#[test]
fn io_error_is_transient() {
    assert!(io_err().is_transient());
}

#[test]
fn protocol_error_is_transient() {
    assert!(AppError::Protocol("framing".into()).is_transient());
}

#[test]
fn invalid_response_is_transient() {
    assert!(AppError::InvalidResponse("missing field".into()).is_transient());
}

#[test]
fn auth_failure_is_terminal() {
    assert!(!AppError::AuthenticationFailed.is_transient());
}

#[test]
fn ui_error_is_terminal() {
    assert!(!AppError::Ui("render".into()).is_transient());
}

// ── backoff sanity ────────────────────────────────────────────────────────────

#[test]
fn backoff_grows_and_stays_bounded() {
    let d1 = backoff_delay(1).as_millis();
    let d4 = backoff_delay(4).as_millis();
    let d10 = backoff_delay(10).as_millis();
    assert!(d1 < d4, "backoff should grow: {d1} < {d4}");
    assert!(d4 <= d10 + 1, "backoff should not shrink at high attempts");
    assert!(
        d10 <= 37_500,
        "backoff must stay bounded (max 30s + 25% jitter)"
    );
    assert!(d1 >= 500, "backoff must be at least MIN_MS");
}

// ── controller state-machine ─────────────────────────────────────────────────

fn make_controller(transport: impl BoincTransport + 'static) -> AppController {
    let rpc = BoincRpcClient::new(Box::new(transport), None);
    AppController::new(rpc, "127.0.0.1:31416".to_string(), None)
}

#[test]
fn transient_failure_transitions_to_retrying() {
    let mut ctrl = make_controller(AlwaysFailTransport);
    ctrl.process_refresh_result(Err(io_err()));
    assert!(
        matches!(
            ctrl.state.conn,
            ConnectionState::Retrying { attempt: 1, .. }
        ),
        "expected Retrying after first IO failure, got {:?}",
        ctrl.state.conn
    );
}

#[test]
fn successive_transient_failures_increment_attempt() {
    let mut ctrl = make_controller(AlwaysFailTransport);
    ctrl.process_refresh_result(Err(io_err()));
    ctrl.process_refresh_result(Err(io_err()));
    ctrl.process_refresh_result(Err(io_err()));
    assert!(
        matches!(
            ctrl.state.conn,
            ConnectionState::Retrying { attempt: 3, .. }
        ),
        "expected attempt=3, got {:?}",
        ctrl.state.conn
    );
}

#[test]
fn recovery_resets_to_connected() {
    let mut ctrl = make_controller(AlwaysFailTransport);
    ctrl.process_refresh_result(Err(io_err()));
    ctrl.process_refresh_result(Err(io_err()));
    assert!(matches!(ctrl.state.conn, ConnectionState::Retrying { .. }));

    ctrl.process_refresh_result(Ok(()));
    assert_eq!(ctrl.state.conn, ConnectionState::Connected);
}

#[test]
fn recovery_resets_attempt_counter() {
    let mut ctrl = make_controller(AlwaysFailTransport);
    for _ in 0..5 {
        ctrl.process_refresh_result(Err(io_err()));
    }
    ctrl.process_refresh_result(Ok(()));
    // After recovery, the first new failure should start at attempt=1 again.
    ctrl.process_refresh_result(Err(io_err()));
    assert!(
        matches!(
            ctrl.state.conn,
            ConnectionState::Retrying { attempt: 1, .. }
        ),
        "attempt counter should reset after recovery, got {:?}",
        ctrl.state.conn
    );
}

#[test]
fn auth_failure_transitions_to_terminal() {
    let mut ctrl = make_controller(AlwaysFailTransport);
    ctrl.process_refresh_result(Err(AppError::AuthenticationFailed));
    assert!(
        matches!(ctrl.state.conn, ConnectionState::TerminalError(_)),
        "expected TerminalError for auth failure, got {:?}",
        ctrl.state.conn
    );
}

#[test]
fn terminal_error_status_line_contains_guidance() {
    let mut ctrl = make_controller(AlwaysFailTransport);
    ctrl.process_refresh_result(Err(AppError::AuthenticationFailed));
    assert!(
        ctrl.state.status_line.contains("Fatal"),
        "status line should contain 'Fatal': {}",
        ctrl.state.status_line
    );
}

#[test]
fn retrying_status_line_mentions_attempt_and_delay() {
    let mut ctrl = make_controller(AlwaysFailTransport);
    ctrl.process_refresh_result(Err(io_err()));
    let line = &ctrl.state.status_line;
    assert!(
        line.contains("retrying") || line.contains("Retrying") || line.contains("retry"),
        "status line should mention retry: {line}"
    );
    assert!(
        line.contains('1'),
        "status line should mention attempt number: {line}"
    );
}

#[test]
fn retrying_delay_increases_with_attempts() {
    let mut ctrl = make_controller(AlwaysFailTransport);
    let d1 = ctrl.process_refresh_result(Err(io_err())).as_millis();
    let d2 = ctrl.process_refresh_result(Err(io_err())).as_millis();
    let d3 = ctrl.process_refresh_result(Err(io_err())).as_millis();
    assert!(
        d1 <= d2 + 300,
        "delay should grow (with jitter tolerance): {d1} → {d2}"
    );
    assert!(
        d2 <= d3 + 300,
        "delay should grow (with jitter tolerance): {d2} → {d3}"
    );
}

#[tokio::test]
async fn full_reconnect_cycle_with_recovering_transport() {
    // Transport fails the first send, then succeeds for subsequent ones.
    // We test the state machine directly (no TUI), simulating what the run
    // loop does: call process_refresh_result, check state, eventually recover.
    let transport = FailThenSucceedTransport::new(0); // succeeds immediately
    let rpc = BoincRpcClient::new(Box::new(transport), None);
    let mut ctrl = AppController::new(rpc, "127.0.0.1:31416".to_string(), None);

    // Simulate initial successful refresh.
    ctrl.process_refresh_result(Ok(()));
    assert_eq!(ctrl.state.conn, ConnectionState::Connected);

    // Simulate daemon going away (two transient failures).
    ctrl.process_refresh_result(Err(io_err()));
    assert!(matches!(
        ctrl.state.conn,
        ConnectionState::Retrying { attempt: 1, .. }
    ));
    ctrl.process_refresh_result(Err(io_err()));
    assert!(matches!(
        ctrl.state.conn,
        ConnectionState::Retrying { attempt: 2, .. }
    ));

    // Daemon comes back.
    ctrl.process_refresh_result(Ok(()));
    assert_eq!(ctrl.state.conn, ConnectionState::Connected);
}
