use crate::boinc::protocol;
use crate::boinc::transport::BoincTransport;
use crate::error::{AppError, AppResult};

/// Stateful BOINC GUI RPC client.
///
/// The client handles:
/// - request framing/response decoding
/// - optional auth handshake (`auth1`/`auth2`)
/// - retry on unauthorized replies when credentials are present
pub struct BoincRpcClient {
    transport: Box<dyn BoincTransport>,
    password: Option<String>,
    authenticated: bool,
}

impl BoincRpcClient {
    /// Creates a new RPC client over the provided transport.
    pub fn new(transport: Box<dyn BoincTransport>, password: Option<String>) -> Self {
        Self {
            transport,
            password,
            authenticated: false,
        }
    }

    /// Runs authentication once when password credentials are configured.
    pub async fn authenticate_if_needed(&mut self) -> AppResult<()> {
        if self.authenticated {
            return Ok(());
        }

        let Some(password) = self.password.clone() else {
            return Ok(());
        };

        let nonce_reply = self.call_raw("<auth1/>").await?;
        let nonce = protocol::parse_auth_nonce(&nonce_reply)?;

        let nonce_hash = protocol::compute_nonce_hash(&nonce, &password);
        let auth2 = format!("<auth2><nonce_hash>{nonce_hash}</nonce_hash></auth2>");
        let auth_reply = self.call_raw(&auth2).await?;
        let authorized = protocol::parse_auth_authorized(&auth_reply)?;
        if !authorized {
            return Err(AppError::AuthenticationFailed);
        }
        self.authenticated = true;
        Ok(())
    }

    /// Calls a BOINC command, wrapping payload inside `<command>...</command>`.
    ///
    /// When `payload` is empty, the command is encoded as a self-closing tag.
    pub async fn call(&mut self, command: &str, payload: &str) -> AppResult<String> {
        self.authenticate_if_needed().await?;
        let inner = if payload.is_empty() {
            format!("<{command}/>")
        } else {
            format!("<{command}>{payload}</{command}>")
        };
        let reply = self.call_raw(&inner).await?;
        if protocol::reply_has_unauthorized(&reply) {
            if self.password.is_some() {
                self.authenticated = false;
                self.authenticate_if_needed().await?;
                let retried = self.call_raw(&inner).await?;
                if protocol::reply_has_unauthorized(&retried) {
                    return Err(AppError::AuthenticationFailed);
                }
                return Ok(retried);
            }
            return Err(AppError::AuthenticationFailed);
        }
        Ok(reply)
    }

    async fn call_raw(&mut self, inner_xml: &str) -> AppResult<String> {
        let framed = protocol::frame_request(inner_xml);
        self.transport.send(&framed).await?;
        let raw = self.transport.receive().await?;
        protocol::parse_response_payload(&raw)
    }
}
