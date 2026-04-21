use crate::boinc::models::{ClientState, Project, RunMode, Task, TaskStatus, Transfer};
use crate::boinc::protocol;
use crate::boinc::rpc_client::BoincRpcClient;
use crate::error::AppResult;

/// Read-only BOINC API facade used by the controller refresh loop.
pub struct BoincReadApi<'a> {
    rpc: &'a mut BoincRpcClient,
}

impl<'a> BoincReadApi<'a> {
    /// Creates a read API facade over the shared RPC client.
    pub fn new(rpc: &'a mut BoincRpcClient) -> Self {
        Self { rpc }
    }

    /// Fetches BOINC project status and maps into typed [`Project`] items.
    pub async fn projects(&mut self) -> AppResult<Vec<Project>> {
        let reply = self.rpc.call("get_project_status", "").await?;
        let parsed = protocol::parse_projects(&reply)?;
        Ok(parsed
            .into_iter()
            .map(|item| Project {
                url: item.url,
                name: item.name,
                suspended_via_gui: item.suspended_via_gui,
                dont_request_more_work: item.dont_request_more_work,
            })
            .collect())
    }

    /// Fetches BOINC task/results and applies UI sort/group ordering.
    ///
    /// Ordering:
    /// 1. ready-to-report
    /// 2. running (by completion descending)
    /// 3. waiting/ready
    pub async fn tasks(&mut self) -> AppResult<Vec<Task>> {
        let reply = self.rpc.call("get_results", "<active_only>0</active_only>").await?;
        let parsed = protocol::parse_tasks(&reply)?;
        let mut tasks: Vec<Task> = parsed
            .into_iter()
            .map(|item| Task {
                project_url: item.project_url,
                name: item.name,
                active_task: item.active_task,
                suspended_via_gui: item.suspended_via_gui,
                fraction_done: item.fraction_done,
                status: map_task_status(item.status),
                elapsed_seconds: item.elapsed_seconds,
                remaining_seconds: item.remaining_seconds,
                report_deadline: item.report_deadline,
                application: item.application,
                checkpoint_cpu_time: item.checkpoint_cpu_time,
                received_time: item.received_time,
                exit_status: item.exit_status,
            })
            .collect();
        tasks.sort_by(compare_tasks_for_view);
        Ok(tasks)
    }

    /// Fetches BOINC transfer list.
    pub async fn transfers(&mut self) -> AppResult<Vec<Transfer>> {
        let reply = self.rpc.call("get_file_transfers", "").await?;
        let parsed = protocol::parse_transfers(&reply)?;
        Ok(parsed
            .into_iter()
            .map(|item| Transfer {
                project_url: item.project_url,
                file_name: item.file_name,
                status: item.status,
                nbytes: item.nbytes,
                bytes_xferred: item.bytes_xferred,
                xfer_speed: item.xfer_speed,
                is_upload: item.is_upload,
                error_msg: item.error_msg,
            })
            .collect())
    }

    /// Fetches BOINC client-wide mode/status values.
    pub async fn client_state(&mut self) -> AppResult<ClientState> {
        let reply = self.rpc.call("get_cc_status", "").await?;
        let parsed = protocol::parse_cc_status(&reply)?;
        Ok(ClientState {
            network_mode: map_run_mode(parsed.network_mode),
            run_mode: map_run_mode(parsed.task_mode),
            gpu_mode: map_run_mode(parsed.gpu_mode),
            messages: Vec::new(),
        })
    }
}

fn map_run_mode(value: Option<String>) -> RunMode {
    match value.as_deref() {
        Some("1") => RunMode::Always,
        Some("2") => RunMode::Auto,
        Some("3") => RunMode::Never,
        _ => RunMode::Auto,
    }
}

fn map_task_status(value: protocol::ParsedTaskStatus) -> TaskStatus {
    match value {
        protocol::ParsedTaskStatus::Running => TaskStatus::Running,
        protocol::ParsedTaskStatus::WaitingToRun => TaskStatus::WaitingToRun,
        protocol::ParsedTaskStatus::ReadyToStart => TaskStatus::ReadyToStart,
        protocol::ParsedTaskStatus::ReadyToReport => TaskStatus::ReadyToReport,
    }
}

fn compare_tasks_for_view(a: &Task, b: &Task) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    let a_rank = status_rank(a.status);
    let b_rank = status_rank(b.status);
    match a_rank.cmp(&b_rank) {
        Ordering::Equal => {
            if a.status == TaskStatus::Running && b.status == TaskStatus::Running {
                let a_pct = a.fraction_done.unwrap_or(0.0);
                let b_pct = b.fraction_done.unwrap_or(0.0);
                match b_pct.partial_cmp(&a_pct).unwrap_or(Ordering::Equal) {
                    Ordering::Equal => a.name.cmp(&b.name),
                    other => other,
                }
            } else {
                a.name.cmp(&b.name)
            }
        }
        other => other,
    }
}

fn status_rank(status: TaskStatus) -> u8 {
    match status {
        TaskStatus::ReadyToReport => 0,
        TaskStatus::Running => 1,
        TaskStatus::WaitingToRun | TaskStatus::ReadyToStart => 2,
    }
}
