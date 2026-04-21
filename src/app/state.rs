use crate::app::actions::FocusPane;
use crate::boinc::models::{ClientState, Project, Task, TaskStatus, Transfer};

/// Tracks the liveness of the BOINC daemon connection.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum ConnectionState {
    /// Last RPC succeeded; normal operation.
    #[default]
    Connected,
    /// Transient failure; will retry after `delay_secs` seconds.
    Retrying { attempt: u32, delay_secs: u64 },
    /// Non-recoverable failure (e.g. wrong password). User action required.
    TerminalError(String),
}

/// In-memory state consumed by renderer and controller logic.
#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub projects: Vec<Project>,
    pub tasks: Vec<Task>,
    pub transfers: Vec<Transfer>,
    pub client_state: ClientState,
    pub focus: FocusPane,
    pub status_line: String,
    pub conn: ConnectionState,
    pub pending_confirmation: Option<String>,
    pub should_quit: bool,
    pub selected_task_idx: usize,
    pub selected_project_idx: usize,
    pub selected_transfer_idx: usize,
    pub show_active_only: bool,
}

impl AppState {
    /// Returns currently selected project URL.
    pub fn selected_project_url(&self) -> Option<&str> {
        self.projects
            .get(self.selected_project_idx)
            .map(|p| p.url.as_str())
    }

    /// Returns `(project_url, task_name)` for the selected task within the filtered list.
    pub fn selected_task(&self) -> Option<(&str, &str)> {
        self.filtered_task_at(self.selected_task_idx)
            .map(|t| (t.project_url.as_str(), t.name.as_str()))
    }

    /// Returns selected task reference within the filtered list.
    pub fn selected_task_ref(&self) -> Option<&Task> {
        self.filtered_task_at(self.selected_task_idx)
    }

    /// Returns selected transfer tuple `(project_url, file_name)` within the filtered list.
    pub fn selected_transfer(&self) -> Option<(&str, &str)> {
        self.filtered_transfer_at(self.selected_transfer_idx)
            .map(|t| (t.project_url.as_str(), t.file_name.as_str()))
    }

    pub fn toggle_active_filter(&mut self) {
        self.show_active_only = !self.show_active_only;
        self.selected_task_idx = 0;
        self.selected_transfer_idx = 0;
    }

    /// Tasks filtered to the currently selected project (all tasks when no project is selected).
    /// When `show_active_only` is set, further limits to Running and ReadyToReport tasks.
    pub fn filtered_tasks(&self) -> Vec<&Task> {
        let url = self.selected_project_url();
        self.tasks
            .iter()
            .filter(|t| url.is_none_or(|u| t.project_url == u))
            .filter(|t| {
                !self.show_active_only
                    || matches!(t.status, TaskStatus::Running | TaskStatus::ReadyToReport)
            })
            .collect()
    }

    /// Transfers filtered to the currently selected project (all transfers when no project is selected).
    /// When `show_active_only` is set, further limits to actively transferring items.
    pub fn filtered_transfers(&self) -> Vec<&Transfer> {
        let url = self.selected_project_url();
        self.transfers
            .iter()
            .filter(|t| url.is_none_or(|u| t.project_url == u))
            .filter(|t| !self.show_active_only || t.xfer_speed.is_some_and(|s| s > 0.0))
            .collect()
    }

    fn filtered_task_at(&self, idx: usize) -> Option<&Task> {
        let url = self.selected_project_url();
        self.tasks
            .iter()
            .filter(|t| url.is_none_or(|u| t.project_url == u))
            .filter(|t| {
                !self.show_active_only
                    || matches!(t.status, TaskStatus::Running | TaskStatus::ReadyToReport)
            })
            .nth(idx)
    }

    fn filtered_transfer_at(&self, idx: usize) -> Option<&Transfer> {
        let url = self.selected_project_url();
        self.transfers
            .iter()
            .filter(|t| url.is_none_or(|u| t.project_url == u))
            .filter(|t| !self.show_active_only || t.xfer_speed.is_some_and(|s| s > 0.0))
            .nth(idx)
    }

    fn filtered_tasks_len(&self) -> usize {
        let url = self.selected_project_url();
        self.tasks
            .iter()
            .filter(|t| url.is_none_or(|u| t.project_url == u))
            .filter(|t| {
                !self.show_active_only
                    || matches!(t.status, TaskStatus::Running | TaskStatus::ReadyToReport)
            })
            .count()
    }

    fn filtered_transfers_len(&self) -> usize {
        let url = self.selected_project_url();
        self.transfers
            .iter()
            .filter(|t| url.is_none_or(|u| t.project_url == u))
            .filter(|t| !self.show_active_only || t.xfer_speed.is_some_and(|s| s > 0.0))
            .count()
    }

    /// Clamps all selection indices after a data refresh.
    pub fn normalize_selection(&mut self) {
        // Clamp project first so filtered lengths below reflect the valid project.
        self.selected_project_idx = clamp_idx(self.selected_project_idx, self.projects.len());
        self.selected_task_idx = clamp_idx(self.selected_task_idx, self.filtered_tasks_len());
        self.selected_transfer_idx =
            clamp_idx(self.selected_transfer_idx, self.filtered_transfers_len());
    }

    pub fn move_task_selection_up(&mut self) {
        self.selected_task_idx = move_up(self.selected_task_idx, self.filtered_tasks_len());
    }

    pub fn move_task_selection_down(&mut self) {
        self.selected_task_idx = move_down(self.selected_task_idx, self.filtered_tasks_len());
    }

    pub fn move_project_selection_up(&mut self) {
        self.selected_project_idx = move_up(self.selected_project_idx, self.projects.len());
        self.selected_task_idx = 0;
        self.selected_transfer_idx = 0;
    }

    pub fn move_project_selection_down(&mut self) {
        self.selected_project_idx = move_down(self.selected_project_idx, self.projects.len());
        self.selected_task_idx = 0;
        self.selected_transfer_idx = 0;
    }

    pub fn move_transfer_selection_up(&mut self) {
        self.selected_transfer_idx =
            move_up(self.selected_transfer_idx, self.filtered_transfers_len());
    }

    pub fn move_transfer_selection_down(&mut self) {
        self.selected_transfer_idx =
            move_down(self.selected_transfer_idx, self.filtered_transfers_len());
    }
}

fn clamp_idx(idx: usize, len: usize) -> usize {
    if len == 0 {
        0
    } else {
        idx.min(len - 1)
    }
}

fn move_up(idx: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    if idx == 0 {
        len - 1
    } else {
        idx - 1
    }
}

fn move_down(idx: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    (idx + 1) % len
}
