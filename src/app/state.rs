use crate::app::actions::FocusPane;
use crate::boinc::models::{ClientState, Project, Task, Transfer};

/// In-memory state consumed by renderer and controller logic.
#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub projects: Vec<Project>,
    pub tasks: Vec<Task>,
    pub transfers: Vec<Transfer>,
    pub client_state: ClientState,
    pub focus: FocusPane,
    pub status_line: String,
    pub pending_confirmation: Option<String>,
    pub should_quit: bool,
    pub selected_task_idx: usize,
}

impl AppState {
    /// Returns currently selected project URL (first project in list).
    pub fn selected_project_url(&self) -> Option<&str> {
        self.projects.first().map(|p| p.url.as_str())
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

    /// Returns currently selected transfer tuple (first transfer in list).
    pub fn selected_transfer(&self) -> Option<(&str, &str)> {
        self.transfers
            .first()
            .map(|t| (t.project_url.as_str(), t.file_name.as_str()))
    }

    /// Ensures selection index remains valid after task list refresh.
    pub fn normalize_selection(&mut self) {
        if self.tasks.is_empty() {
            self.selected_task_idx = 0;
            return;
        }
        if self.selected_task_idx >= self.tasks.len() {
            self.selected_task_idx = self.tasks.len().saturating_sub(1);
        }
    }

    /// Moves task selection up (wrapping at the top).
    pub fn move_task_selection_up(&mut self) {
        if self.tasks.is_empty() {
            self.selected_task_idx = 0;
            return;
        }
        if self.selected_task_idx == 0 {
            self.selected_task_idx = self.tasks.len().saturating_sub(1);
        } else {
            self.selected_task_idx = self.selected_task_idx.saturating_sub(1);
        }
    }

    /// Moves task selection down (wrapping at the bottom).
    pub fn move_task_selection_down(&mut self) {
        if self.tasks.is_empty() {
            self.selected_task_idx = 0;
            return;
        }
        self.selected_task_idx = (self.selected_task_idx + 1) % self.tasks.len();
    }
}
