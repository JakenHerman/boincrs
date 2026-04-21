use crate::boinc::models::ClientState;

pub fn summary(client_state: &ClientState) -> String {
    format!(
        "run:{:?} net:{:?} gpu:{:?} msgs:{}",
        client_state.run_mode,
        client_state.network_mode,
        client_state.gpu_mode,
        client_state.messages.len()
    )
}
