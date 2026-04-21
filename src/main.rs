use boincrs::app::controller::AppController;
use boincrs::boinc::bootstrap::attach_projects_from_env;
use boincrs::boinc::rpc_client::BoincRpcClient;
use boincrs::boinc::transport::TcpBoincTransport;
use boincrs::error::AppResult;

#[tokio::main]
async fn main() -> AppResult<()> {
    let endpoint = std::env::var("BOINCRS_ENDPOINT").unwrap_or_else(|_| "127.0.0.1:31416".to_string());
    let password = load_password_from_env();

    let transport = TcpBoincTransport::connect(endpoint).await?;
    let mut rpc_client = BoincRpcClient::new(Box::new(transport), password);
    let _ = attach_projects_from_env(&mut rpc_client).await?;
    let mut controller = AppController::new(rpc_client);
    controller.run().await
}

fn load_password_from_env() -> Option<String> {
    if let Ok(password) = std::env::var("BOINCRS_PASSWORD") {
        let trimmed = password.trim().to_string();
        if !trimmed.is_empty() {
            return Some(trimmed);
        }
    }

    if let Ok(path) = std::env::var("BOINCRS_PASSWORD_FILE") {
        if let Ok(contents) = std::fs::read_to_string(path) {
            let trimmed = contents.trim().to_string();
            if !trimmed.is_empty() {
                return Some(trimmed);
            }
        }
    }
    None
}
