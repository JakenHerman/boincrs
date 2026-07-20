use boincrs::app::controller::AppController;
use boincrs::boinc::bootstrap::attach_projects_from_env;
use boincrs::boinc::rpc_client::BoincRpcClient;
use boincrs::boinc::transport::TcpBoincTransport;
use boincrs::error::AppResult;
use clap::Parser;

const DEFAULT_ENDPOINT: &str = "127.0.0.1:31416";

const ENV_HELP: &str = "\
boincrs is configured through the environment (and an optional .env file in the
working directory):

  BOINCRS_ENDPOINT        BOINC GUI RPC address (default 127.0.0.1:31416)
  BOINCRS_PASSWORD        GUI RPC password
  BOINCRS_PASSWORD_FILE   path to a file containing the GUI RPC password

The password is read only from the environment, never from a flag, so it does
not leak into shell history or the process list.

Full guide: https://jakenherman.github.io/boincrs";

/// A fast, keyboard-first terminal UI for a local BOINC client.
#[derive(Parser)]
#[command(name = "boincrs", version, after_help = ENV_HELP)]
struct Cli {
    /// BOINC GUI RPC address to connect to (overrides BOINCRS_ENDPOINT).
    #[arg(long, value_name = "HOST:PORT")]
    endpoint: Option<String>,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    // clap handles --help/--version and rejects unknown arguments, exiting
    // before any network work so packaging smoke tests can run without a
    // BOINC daemon.
    let cli = Cli::parse();
    load_dotenv();

    let endpoint = cli
        .endpoint
        .or_else(|| std::env::var("BOINCRS_ENDPOINT").ok())
        .unwrap_or_else(|| DEFAULT_ENDPOINT.to_string());
    let password = load_password_from_env();

    let transport = TcpBoincTransport::connect(endpoint.clone()).await?;
    let mut rpc_client = BoincRpcClient::new(Box::new(transport), password.clone());
    let bootstrap_report = attach_projects_from_env(&mut rpc_client).await?;
    let mut controller = AppController::new(rpc_client, endpoint, password);
    if !bootstrap_report.attached.is_empty() || bootstrap_report.profile_name.is_some() {
        let profile_part = bootstrap_report
            .profile_name
            .as_deref()
            .map(|n| format!(" profile:{n}"))
            .unwrap_or_default();
        let skipped_part = if bootstrap_report.skipped.is_empty() {
            String::new()
        } else {
            format!(" skipped:{}", bootstrap_report.skipped.len())
        };
        controller.state.status_line = format!(
            "Bootstrap: attached {}{}{}",
            bootstrap_report.attached.len(),
            profile_part,
            skipped_part,
        );
    }
    controller.run().await
}

fn load_dotenv() {
    let Ok(contents) = std::fs::read_to_string(".env") else {
        return;
    };
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"').trim_matches('\'');
            if std::env::var(key).is_err() {
                unsafe { std::env::set_var(key, value) };
            }
        }
    }
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
