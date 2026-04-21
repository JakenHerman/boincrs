use crate::app::actions::FocusPane;
use crate::boinc::models::{ClientState, Project, Task, Transfer};

/// Tracks the liveness of the BOINC daemon connection.
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    /// Last RPC succeeded; normal operation.
    Connected,
    /// Transient failure; will retry after `delay_secs` seconds.
    Retrying { attempt: u32, delay_secs: u64 },
    /// Non-recoverable failure (e.g. wrong password). User action required.
    TerminalError(String),
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState::Connected
    }
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
}

impl AppState {
    /// Returns currently selected project URL.
    pub fn selected_project_url(&self) -> Option<&str> {
        self.projects
            .get(self.selected_project_idx)
            .map(|p| p.url.as_str())
    }

    /// Returns `(project_url, task_name)` for selected task.
    pub fn selected_task(&self) -> Option<(&str, &str)> {
        self.tasks
            .get(self.selected_task_idx)
            .map(|t| (t.project_url.as_str(), t.name.as_str()))
    }

    /// Returns selected task reference.
    pub fn selected_task_ref(&self) -> Option<&Task> {
        self.tasks.get(self.selected_task_idx)
    }

    /// Returns selected transfer tuple `(project_url, file_name)`.
    pub fn selected_transfer(&self) -> Option<(&str, &str)> {
        self.transfers
            .get(self.selected_transfer_idx)
            .map(|t| (t.project_url.as_str(), t.file_name.as_str()))
    }

    /// Clamps all selection indices after a data refresh.
    pub fn normalize_selection(&mut self) {
        self.selected_task_idx = clamp_idx(self.selected_task_idx, self.tasks.len());
        self.selected_project_idx = clamp_idx(self.selected_project_idx, self.projects.len());
        self.selected_transfer_idx = clamp_idx(self.selected_transfer_idx, self.transfers.len());
    }

    pub fn move_task_selection_up(&mut self) {
        self.selected_task_idx = move_up(self.selected_task_idx, self.tasks.len());
    }

    pub fn move_task_selection_down(&mut self) {
        self.selected_task_idx = move_down(self.selected_task_idx, self.tasks.len());
    }

    pub fn move_project_selection_up(&mut self) {
        self.selected_project_idx = move_up(self.selected_project_idx, self.projects.len());
    }

    pub fn move_project_selection_down(&mut self) {
        self.selected_project_idx = move_down(self.selected_project_idx, self.projects.len());
    }

    pub fn move_transfer_selection_up(&mut self) {
        self.selected_transfer_idx = move_up(self.selected_transfer_idx, self.transfers.len());
    }

    pub fn move_transfer_selection_down(&mut self) {
        self.selected_transfer_idx = move_down(self.selected_transfer_idx, self.transfers.len());
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
