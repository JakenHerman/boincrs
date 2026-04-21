use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::error::AppResult;

const EOM: u8 = 0x03;

/// Abstract transport used by the BOINC RPC client.
///
/// Implementations can wrap TCP sockets (default) or mocks for tests.
#[async_trait]
pub trait BoincTransport: Send {
    /// Sends a fully framed request payload.
    async fn send(&mut self, payload: &[u8]) -> AppResult<()>;
    /// Receives bytes until BOINC end-of-message marker is observed.
    async fn receive(&mut self) -> AppResult<Vec<u8>>;
}

/// TCP implementation of [`BoincTransport`].
pub struct TcpBoincTransport {
    stream: TcpStream,
}

impl TcpBoincTransport {
    /// Connects to BOINC GUI RPC endpoint (for example `127.0.0.1:31416`).
    pub async fn connect(endpoint: String) -> AppResult<Self> {
        let stream = TcpStream::connect(endpoint).await?;
        Ok(Self { stream })
    }
}

#[async_trait]
impl BoincTransport for TcpBoincTransport {
    async fn send(&mut self, payload: &[u8]) -> AppResult<()> {
        self.stream.write_all(payload).await?;
        self.stream.flush().await?;
        Ok(())
    }

    async fn receive(&mut self) -> AppResult<Vec<u8>> {
        let mut out = Vec::new();
        loop {
            let mut buf = [0u8; 1024];
            let n = self.stream.read(&mut buf).await?;
            if n == 0 {
                break;
            }
            out.extend_from_slice(&buf[..n]);
            if out.contains(&EOM) {
                break;
            }
        }
        Ok(out)
    }
}
