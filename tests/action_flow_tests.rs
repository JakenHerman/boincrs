use async_trait::async_trait;
use boincrs::boinc::api::write::BoincWriteApi;
use boincrs::boinc::rpc_client::BoincRpcClient;
use boincrs::boinc::transport::BoincTransport;
use boincrs::error::AppResult;

struct MockTransport {
    responses: Vec<Vec<u8>>,
    writes: Vec<Vec<u8>>,
}

impl MockTransport {
    fn new(responses: Vec<Vec<u8>>) -> Self {
        Self {
            responses,
            writes: Vec::new(),
        }
    }
}

#[async_trait]
impl BoincTransport for MockTransport {
    async fn send(&mut self, payload: &[u8]) -> AppResult<()> {
        self.writes.push(payload.to_vec());
        Ok(())
    }

    async fn receive(&mut self) -> AppResult<Vec<u8>> {
        if self.responses.is_empty() {
            return Ok(b"<boinc_gui_rpc_reply><success/></boinc_gui_rpc_reply>\x03".to_vec());
        }
        Ok(self.responses.remove(0))
    }
}

#[tokio::test]
async fn write_api_dispatches_project_command() {
    let transport = MockTransport::new(vec![
        b"<boinc_gui_rpc_reply><success/></boinc_gui_rpc_reply>\x03".to_vec(),
    ]);
    let mut client = BoincRpcClient::new(Box::new(transport), None);
    let mut api = BoincWriteApi::new(&mut client);
    let reply = api
        .project_suspend("https://example.invalid")
        .await
        .expect("suspend should succeed");
    assert!(reply.contains("boinc_gui_rpc_reply"));
}
