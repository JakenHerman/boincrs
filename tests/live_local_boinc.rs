use boincrs::boinc::api::read::BoincReadApi;
use boincrs::boinc::rpc_client::BoincRpcClient;
use boincrs::boinc::transport::TcpBoincTransport;

#[tokio::test]
#[ignore = "requires local BOINC daemon on 127.0.0.1:31416"]
async fn live_local_auth_and_reads() {
    let endpoint =
        std::env::var("BOINCRS_ENDPOINT").unwrap_or_else(|_| "127.0.0.1:31416".to_string());
    let password_path = std::env::var("BOINCRS_PASSWORD_FILE")
        .unwrap_or_else(|_| "/etc/boinc-client/gui_rpc_auth.cfg".to_string());
    let password = std::fs::read_to_string(password_path)
        .expect("must read BOINC gui rpc password file")
        .trim()
        .to_string();

    let transport = TcpBoincTransport::connect(endpoint)
        .await
        .expect("must connect to local BOINC RPC endpoint");
    let mut client = BoincRpcClient::new(Box::new(transport), Some(password));

    // Verify auth and full read surface does not fail against real daemon.
    client
        .authenticate_if_needed()
        .await
        .expect("auth handshake should succeed");

    let mut read_api = BoincReadApi::new(&mut client);
    let projects = read_api
        .projects()
        .await
        .expect("projects call should succeed");
    let tasks = read_api.tasks().await.expect("tasks call should succeed");
    let transfers = read_api
        .transfers()
        .await
        .expect("transfers call should succeed");
    let status = read_api
        .client_state()
        .await
        .expect("client status call should succeed");

    eprintln!(
        "live rpc ok: projects={} tasks={} transfers={} run_mode={:?}",
        projects.len(),
        tasks.len(),
        transfers.len(),
        status.run_mode
    );
}
