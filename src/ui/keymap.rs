use crossterm::event::{KeyCode, KeyEvent};

use crate::app::actions::UserAction;
use crate::boinc::models::RunMode;

pub fn map_key_to_action(key: KeyEvent) -> Option<UserAction> {
    match key.code {
        KeyCode::Char('q') => Some(UserAction::Quit),
        KeyCode::Char('r') => Some(UserAction::RefreshNow),
        KeyCode::Tab => Some(UserAction::CyclePane),
        KeyCode::Char('k') | KeyCode::Up => Some(UserAction::MoveUp),
        KeyCode::Char('j') | KeyCode::Down => Some(UserAction::MoveDown),
        KeyCode::Char('y') => Some(UserAction::ConfirmPending),
        KeyCode::Char('n') => Some(UserAction::CancelPending),
        KeyCode::Char('u') => Some(UserAction::ProjectUpdate),
        KeyCode::Char('s') => Some(UserAction::ProjectSuspend),
        KeyCode::Char('c') => Some(UserAction::ProjectResume),
        KeyCode::Char('w') => Some(UserAction::ProjectNoMoreWork),
        KeyCode::Char('a') => Some(UserAction::ProjectAllowMoreWork),
        KeyCode::Char('x') => Some(UserAction::ProjectReset),
        KeyCode::Char('d') => Some(UserAction::ProjectDetach),
        KeyCode::Char('t') => Some(UserAction::TaskSuspend),
        KeyCode::Char('g') => Some(UserAction::TaskResume),
        KeyCode::Char('b') => Some(UserAction::TaskAbort),
        KeyCode::Char('f') => Some(UserAction::TransferRetry),
        KeyCode::Char('1') => Some(UserAction::SetRunMode(RunMode::Always)),
        KeyCode::Char('2') => Some(UserAction::SetRunMode(RunMode::Auto)),
        KeyCode::Char('3') => Some(UserAction::SetRunMode(RunMode::Never)),
        KeyCode::Char('4') => Some(UserAction::SetNetworkMode(RunMode::Always)),
        KeyCode::Char('5') => Some(UserAction::SetNetworkMode(RunMode::Auto)),
        KeyCode::Char('6') => Some(UserAction::SetNetworkMode(RunMode::Never)),
        KeyCode::Char('7') => Some(UserAction::SetGpuMode(RunMode::Always)),
        KeyCode::Char('8') => Some(UserAction::SetGpuMode(RunMode::Auto)),
        KeyCode::Char('9') => Some(UserAction::SetGpuMode(RunMode::Never)),
        KeyCode::Char('D') => Some(UserAction::SaveDiagnostics),
        _ => None,
    }
}
