use boincrs::boinc::api::read::BoincReadApi;
use boincrs::boinc::api::write::BoincWriteApi;
use boincrs::boinc::rpc_client::BoincRpcClient;
use boincrs::boinc::transport::TcpBoincTransport;

fn required_env(key: &str) -> String {
    std::env::var(key)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| panic!("missing required env var: {key}"))
}

#[tokio::test]
#[ignore = "requires live BOINC and valid account keys"]
async fn live_primegrid_and_asteroids_attach_and_visibility() {
    let endpoint = std::env::var("BOINCRS_ENDPOINT").unwrap_or_else(|_| "127.0.0.1:31416".to_string());
    let password_file = std::env::var("BOINCRS_PASSWORD_FILE")
        .unwrap_or_else(|_| "/etc/boinc-client/gui_rpc_auth.cfg".to_string());
    let password = std::fs::read_to_string(password_file)
        .expect("must read BOINC gui rpc password file")
        .trim()
        .to_string();

    let primegrid_key = required_env("BOINCRS_PRIMEGRID_ACCOUNT_KEY");
    let asteroids_key = required_env("BOINCRS_ASTEROIDS_ACCOUNT_KEY");

    let transport = TcpBoincTransport::connect(endpoint)
        .await
        .expect("must connect to local BOINC RPC endpoint");
    let mut client = BoincRpcClient::new(Box::new(transport), Some(password));
    client
        .authenticate_if_needed()
        .await
        .expect("auth handshake should succeed");

    {
        let mut write_api = BoincWriteApi::new(&mut client);
        let _ = write_api
            .project_attach("https://www.primegrid.com/", primegrid_key.as_str())
            .await
            .expect("primegrid attach should succeed");
        let _ = write_api
            .project_attach("https://asteroidsathome.net/boinc/", asteroids_key.as_str())
            .await
            .expect("asteroids attach should succeed");
        let _ = write_api
            .project_update("https://www.primegrid.com/")
            .await
            .expect("primegrid update should succeed");
        let _ = write_api
            .project_update("https://asteroidsathome.net/boinc/")
            .await
            .expect("asteroids update should succeed");
    }

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let mut read_api = BoincReadApi::new(&mut client);
    let projects = read_api.projects().await.expect("projects read should succeed");
    let tasks = read_api.tasks().await.expect("tasks read should succeed");

    let has_primegrid = projects.iter().any(|p| p.url.contains("primegrid.com"));
    let has_asteroids = projects.iter().any(|p| p.url.contains("asteroidsathome.net"));
    assert!(has_primegrid, "primegrid should appear in project list");
    assert!(has_asteroids, "asteroids should appear in project list");

    let has_target_tasks = tasks.iter().any(|t| {
        t.project_url.contains("primegrid.com") || t.project_url.contains("asteroidsathome.net")
    });
    eprintln!(
        "beta visibility: projects={} tasks={} target_tasks_present={}",
        projects.len(),
        tasks.len(),
        has_target_tasks
    );
}
