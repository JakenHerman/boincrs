use std::io::Write;

use boincrs::app::controller::AppController;
use boincrs::boinc::bootstrap::attach_projects_from_env;
use boincrs::boinc::rpc_client::BoincRpcClient;
use boincrs::boinc::transport::TcpBoincTransport;
use boincrs::error::AppResult;

const HELP: &str = "\
boincrs \u{2014} a fast, keyboard-first terminal UI for a local BOINC client

Usage:
    boincrs [options]

Options:
    -h, --help       Print this help and exit
    -V, --version    Print version information and exit

boincrs is configured through the environment (and an optional .env file in the
working directory). Common variables:

    BOINCRS_ENDPOINT        BOINC GUI RPC address (default 127.0.0.1:31416)
    BOINCRS_PASSWORD        GUI RPC password
    BOINCRS_PASSWORD_FILE   path to a file containing the GUI RPC password

Full guide: https://jakenherman.github.io/boincrs
";

#[tokio::main]
async fn main() -> AppResult<()> {
    handle_cli_flags();
    load_dotenv();
    let endpoint =
        std::env::var("BOINCRS_ENDPOINT").unwrap_or_else(|_| "127.0.0.1:31416".to_string());
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

/// Handle `--help` / `--version` before any network work.
///
/// Exits the process when such a flag (or an unrecognized argument) is present;
/// returns normally when there is nothing to do and the TUI should launch.
/// Keeping this ahead of any connection attempt means packaging smoke tests
/// (Homebrew `test do`, Chocolatey validation) can query the binary without a
/// running BOINC daemon.
fn handle_cli_flags() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        return;
    }
    // `--help` and `--version` take precedence over anything else on the line.
    if args.iter().any(|a| a == "-h" || a == "--help") {
        exit_with(0, HELP);
    }
    if args.iter().any(|a| a == "-V" || a == "--version") {
        exit_with(0, &format!("boincrs {}\n", env!("CARGO_PKG_VERSION")));
    }
    // boincrs takes no positional arguments, so anything left is unrecognized.
    let unknown = &args[0];
    let msg = format!(
        "boincrs: unrecognized argument '{unknown}'\n\
         Try 'boincrs --help' for usage.\n"
    );
    let mut err = std::io::stderr();
    let _ = err.write_all(msg.as_bytes());
    let _ = err.flush();
    std::process::exit(2);
}

/// Write `message` to stdout, flush it, and exit with `code`.
///
/// Uses an explicit flush so output is not lost when stdout is block-buffered
/// (e.g. piped into another process), which `std::process::exit` would
/// otherwise discard.
fn exit_with(code: i32, message: &str) -> ! {
    let mut out = std::io::stdout();
    let _ = out.write_all(message.as_bytes());
    let _ = out.flush();
    std::process::exit(code);
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
