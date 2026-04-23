use thiserror::Error;

/// Common result type used throughout `boincrs`.
pub type AppResult<T> = Result<T, AppError>;

/// Top-level error type used by transport, protocol, and UI layers.
#[derive(Debug, Error)]
pub enum AppError {
    /// Underlying I/O failure (network/socket/terminal).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// Protocol-level request/response handling failure.
    #[error("RPC protocol error: {0}")]
    Protocol(String),
    /// BOINC GUI RPC authentication rejected credentials.
    #[error("RPC authentication failed")]
    AuthenticationFailed,
    /// BOINC response payload was malformed or missing required fields.
    #[error("Invalid BOINC response: {0}")]
    InvalidResponse(String),
    /// UI/runtime interaction error.
    #[error("UI error: {0}")]
    Ui(String),
    /// Configuration input was invalid (template slug, profile, env var).
    #[error("configuration error: {0}")]
    Config(String),
}

impl AppError {
    /// Returns `true` for errors that may resolve on retry (network/socket/framing).
    /// Returns `false` for errors that require human action (wrong password, etc.).
    pub fn is_transient(&self) -> bool {
        match self {
            AppError::Io(_) => true,
            AppError::Protocol(_) => true,
            AppError::InvalidResponse(_) => true,
            AppError::AuthenticationFailed => false,
            AppError::Ui(_) => false,
            AppError::Config(_) => false,
        }
    }
}
