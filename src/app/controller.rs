use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crossterm::event::{self, Event};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::app::actions::{FocusPane, UserAction};
use crate::app::state::AppState;
use crate::boinc::api::read::BoincReadApi;
use crate::boinc::api::write::BoincWriteApi;
use crate::boinc::rpc_client::BoincRpcClient;
use crate::error::AppResult;
use crate::ui;

pub struct AppController {
    rpc: BoincRpcClient,
    state: AppState,
    pending_action: Option<UserAction>,
}

impl AppController {
    pub fn new(rpc: BoincRpcClient) -> Self {
        Self {
            rpc,
            state: AppState::default(),
            pending_action: None,
        }
    }

    pub async fn run(&mut self) -> AppResult<()> {
        let mut stdout = std::io::stdout();
        enable_raw_mode()?;
        stdout.execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        self.refresh().await?;
        let mut ticker = tokio::time::interval(Duration::from_secs(2));
        self.state.status_line = "Connected. Refreshing every 2s.".to_string();

        while !self.state.should_quit {
            terminal.draw(|f| ui::layout::draw(f, &self.state))?;

            tokio::select! {
                _ = ticker.tick() => {
                    if let Err(err) = self.refresh().await {
                        self.state.status_line = format!("refresh failed: {err}");
                    }
                }
                _ = tokio::time::sleep(Duration::from_millis(50)) => {
                    if event::poll(Duration::from_millis(0))? {
                        if let Event::Key(key) = event::read()? {
                            if let Some(action) = ui::keymap::map_key_to_action(key) {
                                self.handle_action(action).await;
                            }
                        }
                    }
                }
            }
        }

        disable_raw_mode()?;
        let mut stdout = std::io::stdout();
        stdout.execute(LeaveAlternateScreen)?;
        Ok(())
    }

    async fn refresh(&mut self) -> AppResult<()> {
        let mut api = BoincReadApi::new(&mut self.rpc);
        self.state.projects = api.projects().await?;
        self.state.tasks = api.tasks().await?;
        self.state.transfers = api.transfers().await?;
        self.state.client_state = api.client_state().await?;
        self.state.normalize_selection();
        Ok(())
    }

    async fn handle_action(&mut self, action: UserAction) {
        if matches!(
            action,
            UserAction::ConfirmPending | UserAction::CancelPending
        ) {
            self.handle_confirmation(action).await;
            return;
        }
        if action.is_destructive() {
            self.pending_action = Some(action.clone());
            self.state.pending_confirmation = Some(format!("{action:?}"));
            return;
        }
        self.execute_action(action).await;
    }

    async fn handle_confirmation(&mut self, action: UserAction) {
        match action {
            UserAction::ConfirmPending => {
                if let Some(pending) = self.pending_action.clone() {
                    self.state.pending_confirmation = None;
                    self.pending_action = None;
                    self.execute_action(pending).await;
                }
            }
            UserAction::CancelPending => {
                self.state.pending_confirmation = None;
                self.pending_action = None;
                self.state.status_line = "Cancelled pending action".to_string();
            }
            _ => {}
        }
    }

    async fn execute_action(&mut self, action: UserAction) {
        match action {
            UserAction::Quit => self.state.should_quit = true,
            UserAction::RefreshNow => {
                if let Err(err) = self.refresh().await {
                    self.state.status_line = format!("refresh failed: {err}");
                } else {
                    self.state.status_line = "refresh completed".to_string();
                }
            }
            UserAction::CyclePane => {
                self.state.focus = self.state.focus.next();
            }
            UserAction::MoveUp => self.navigate_selection(true),
            UserAction::MoveDown => self.navigate_selection(false),
            UserAction::SaveDiagnostics => self.save_diagnostics(),
            _ => {
                if let Err(err) = self.dispatch_rpc_action(action).await {
                    self.state.status_line = format!("action failed: {err}");
                } else {
                    self.state.status_line = "action succeeded".to_string();
                    let _ = self.refresh().await;
                }
            }
        }
    }

    async fn dispatch_rpc_action(&mut self, action: UserAction) -> AppResult<()> {
        let mut api = BoincWriteApi::new(&mut self.rpc);
        match action {
            UserAction::ProjectUpdate => {
                if let Some(url) = self.state.selected_project_url() {
                    let _ = api.project_update(url).await?;
                }
            }
            UserAction::ProjectSuspend => {
                if let Some(url) = self.state.selected_project_url() {
                    let _ = api.project_suspend(url).await?;
                }
            }
            UserAction::ProjectResume => {
                if let Some(url) = self.state.selected_project_url() {
                    let _ = api.project_resume(url).await?;
                }
            }
            UserAction::ProjectNoMoreWork => {
                if let Some(url) = self.state.selected_project_url() {
                    let _ = api.project_no_more_work(url).await?;
                }
            }
            UserAction::ProjectAllowMoreWork => {
                if let Some(url) = self.state.selected_project_url() {
                    let _ = api.project_allow_more_work(url).await?;
                }
            }
            UserAction::ProjectReset => {
                if let Some(url) = self.state.selected_project_url() {
                    let _ = api.project_reset(url).await?;
                }
            }
            UserAction::ProjectDetach => {
                if let Some(url) = self.state.selected_project_url() {
                    let _ = api.project_detach(url).await?;
                }
            }
            UserAction::TaskSuspend => {
                if let Some((project_url, task_name)) = self.state.selected_task() {
                    let _ = api.task_suspend(project_url, task_name).await?;
                }
            }
            UserAction::TaskResume => {
                if let Some((project_url, task_name)) = self.state.selected_task() {
                    let _ = api.task_resume(project_url, task_name).await?;
                }
            }
            UserAction::TaskAbort => {
                if let Some((project_url, task_name)) = self.state.selected_task() {
                    let _ = api.task_abort(project_url, task_name).await?;
                }
            }
            UserAction::TransferRetry => {
                if let Some((project_url, file_name)) = self.state.selected_transfer() {
                    let _ = api.transfer_retry(project_url, file_name).await?;
                }
            }
            UserAction::SetNetworkMode(mode) => {
                let _ = api.set_network_mode(mode, 0).await?;
            }
            UserAction::SetRunMode(mode) => {
                let _ = api.set_run_mode(mode, 0).await?;
            }
            UserAction::SetGpuMode(mode) => {
                let _ = api.set_gpu_mode(mode, 0).await?;
            }
            UserAction::ConfirmPending
            | UserAction::CancelPending
            | UserAction::RefreshNow
            | UserAction::CyclePane
            | UserAction::MoveUp
            | UserAction::MoveDown
            | UserAction::SaveDiagnostics
            | UserAction::Quit => {}
        }
        Ok(())
    }

    fn save_diagnostics(&mut self) {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let path = format!("boincrs-diag-{ts}.txt");
        let s = &self.state;
        let mut out = format!(
            "boincrs diagnostics — epoch {ts}\n\
             client: run={:?} net={:?} gpu={:?} msgs={}\n\n",
            s.client_state.run_mode,
            s.client_state.network_mode,
            s.client_state.gpu_mode,
            s.client_state.messages.len(),
        );
        out.push_str(&format!("=== projects ({}) ===\n", s.projects.len()));
        for p in &s.projects {
            out.push_str(&format!(
                "  {} | url:{} | suspended:{} | nmw:{}\n",
                p.name, p.url, p.suspended_via_gui, p.dont_request_more_work
            ));
        }
        out.push_str(&format!("\n=== tasks ({}) ===\n", s.tasks.len()));
        for t in &s.tasks {
            out.push_str(&format!(
                "  {} | project:{} | status:{:?} | pct:{} | elapsed:{:?}s | remaining:{:?}s | deadline:{:?} | chkpt:{:?}s | exit:{:?} | app:{:?}\n",
                t.name, t.project_url, t.status,
                t.fraction_done.map(|v| format!("{:.1}%", v * 100.0)).unwrap_or_else(|| "n/a".into()),
                t.elapsed_seconds, t.remaining_seconds, t.report_deadline,
                t.checkpoint_cpu_time, t.exit_status, t.application,
            ));
        }
        out.push_str(&format!("\n=== transfers ({}) ===\n", s.transfers.len()));
        for tr in &s.transfers {
            out.push_str(&format!(
                "  {} | project:{} | upload:{} | status:{} | bytes:{:?}/{:?} | speed:{:?} | err:{:?}\n",
                tr.file_name, tr.project_url, tr.is_upload, tr.status,
                tr.bytes_xferred, tr.nbytes, tr.xfer_speed, tr.error_msg,
            ));
        }
        match std::fs::write(&path, &out) {
            Ok(()) => self.state.status_line = format!("diagnostics saved to {path}"),
            Err(e) => self.state.status_line = format!("failed to save diagnostics: {e}"),
        }
    }

    fn navigate_selection(&mut self, up: bool) {
        match self.state.focus {
            FocusPane::Projects => {
                if up {
                    self.state.move_project_selection_up();
                } else {
                    self.state.move_project_selection_down();
                }
            }
            FocusPane::Tasks => {
                if up {
                    self.state.move_task_selection_up();
                } else {
                    self.state.move_task_selection_down();
                }
            }
            FocusPane::Transfers => {
                if up {
                    self.state.move_transfer_selection_up();
                } else {
                    self.state.move_transfer_selection_down();
                }
            }
        }
    }
}
