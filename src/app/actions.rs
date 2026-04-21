use crate::boinc::models::RunMode;

/// Focusable pane identifiers in the main TUI layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusPane {
    #[default]
    Projects,
    Tasks,
    Transfers,
}

impl FocusPane {
    /// Cycles focus in the order: Projects -> Tasks -> Transfers -> Projects.
    pub fn next(self) -> Self {
        match self {
            Self::Projects => Self::Tasks,
            Self::Tasks => Self::Transfers,
            Self::Transfers => Self::Projects,
        }
    }
}

/// User-triggered actions from keyboard input.
#[derive(Debug, Clone)]
pub enum UserAction {
    RefreshNow,
    CyclePane,
    MoveUp,
    MoveDown,
    Quit,
    ConfirmPending,
    CancelPending,
    ProjectUpdate,
    ProjectSuspend,
    ProjectResume,
    ProjectNoMoreWork,
    ProjectAllowMoreWork,
    ProjectReset,
    ProjectDetach,
    TaskSuspend,
    TaskResume,
    TaskAbort,
    TransferRetry,
    SetNetworkMode(RunMode),
    SetRunMode(RunMode),
    SetGpuMode(RunMode),
    SaveDiagnostics,
}

impl UserAction {
    /// Returns true if action should require explicit confirmation.
    pub fn is_destructive(&self) -> bool {
        matches!(
            self,
            Self::ProjectReset | Self::ProjectDetach | Self::TaskAbort
        )
    }
}
