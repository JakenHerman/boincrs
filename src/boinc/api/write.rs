use crate::boinc::models::RunMode;
use crate::boinc::rpc_client::BoincRpcClient;
use crate::error::AppResult;

/// BOINC write/control API facade.
///
/// Exposes high-level methods for project, task, transfer, and mode commands.
pub struct BoincWriteApi<'a> {
    rpc: &'a mut BoincRpcClient,
}

impl<'a> BoincWriteApi<'a> {
    /// Creates a write API facade over the shared RPC client.
    pub fn new(rpc: &'a mut BoincRpcClient) -> Self {
        Self { rpc }
    }

    /// Sends a project-level command carrying only `project_url`.
    pub async fn project_simple(&mut self, command: &str, project_url: &str) -> AppResult<String> {
        let payload = format!("<project_url>{project_url}</project_url>");
        self.rpc.call(command, &payload).await
    }

    /// Requests scheduler update for a project.
    pub async fn project_update(&mut self, project_url: &str) -> AppResult<String> {
        self.project_simple("project_update", project_url).await
    }

    /// Suspends all tasks for a project.
    pub async fn project_suspend(&mut self, project_url: &str) -> AppResult<String> {
        self.project_simple("project_suspend", project_url).await
    }

    /// Resumes a suspended project.
    pub async fn project_resume(&mut self, project_url: &str) -> AppResult<String> {
        self.project_simple("project_resume", project_url).await
    }

    /// Prevents project from requesting new work.
    pub async fn project_no_more_work(&mut self, project_url: &str) -> AppResult<String> {
        self.project_simple("project_nomorework", project_url).await
    }

    /// Allows project to request new work again.
    pub async fn project_allow_more_work(&mut self, project_url: &str) -> AppResult<String> {
        self.project_simple("project_allowmorework", project_url).await
    }

    /// Resets project (destructive).
    pub async fn project_reset(&mut self, project_url: &str) -> AppResult<String> {
        self.project_simple("project_reset", project_url).await
    }

    /// Detaches project (destructive).
    pub async fn project_detach(&mut self, project_url: &str) -> AppResult<String> {
        self.project_simple("project_detach", project_url).await
    }

    /// Attaches a BOINC project using account authenticator key.
    pub async fn project_attach(&mut self, project_url: &str, authenticator: &str) -> AppResult<String> {
        let payload = format!(
            "<project_url>{project_url}</project_url><authenticator>{authenticator}</authenticator>"
        );
        self.rpc.call("project_attach", &payload).await
    }

    /// Sends a task-level action for a specific project/result.
    pub async fn task_action(&mut self, command: &str, project_url: &str, task_name: &str) -> AppResult<String> {
        let payload = format!("<project_url>{project_url}</project_url><name>{task_name}</name>");
        self.rpc.call(command, &payload).await
    }

    /// Suspends a task.
    pub async fn task_suspend(&mut self, project_url: &str, task_name: &str) -> AppResult<String> {
        self.task_action("result_suspend", project_url, task_name).await
    }

    /// Resumes a task.
    pub async fn task_resume(&mut self, project_url: &str, task_name: &str) -> AppResult<String> {
        self.task_action("result_resume", project_url, task_name).await
    }

    /// Aborts a task (destructive).
    pub async fn task_abort(&mut self, project_url: &str, task_name: &str) -> AppResult<String> {
        self.task_action("result_abort", project_url, task_name).await
    }

    /// Retries a failed transfer.
    pub async fn transfer_retry(&mut self, project_url: &str, file_name: &str) -> AppResult<String> {
        let payload = format!("<project_url>{project_url}</project_url><filename>{file_name}</filename>");
        self.rpc.call("retry_file_transfer", &payload).await
    }

    async fn set_mode(&mut self, command: &str, mode: RunMode, duration_secs: u64) -> AppResult<String> {
        let payload = format!(
            "<{mode_tag}/><duration>{duration_secs}</duration>",
            mode_tag = mode.as_boinc_tag()
        );
        self.rpc.call(command, &payload).await
    }

    /// Sets BOINC CPU run mode.
    pub async fn set_run_mode(&mut self, mode: RunMode, duration_secs: u64) -> AppResult<String> {
        self.set_mode("set_run_mode", mode, duration_secs).await
    }

    /// Sets BOINC network mode.
    pub async fn set_network_mode(&mut self, mode: RunMode, duration_secs: u64) -> AppResult<String> {
        self.set_mode("set_network_mode", mode, duration_secs).await
    }

    /// Sets BOINC GPU mode.
    pub async fn set_gpu_mode(&mut self, mode: RunMode, duration_secs: u64) -> AppResult<String> {
        self.set_mode("set_gpu_mode", mode, duration_secs).await
    }
}
