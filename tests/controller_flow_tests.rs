use boincrs::app::actions::{FocusPane, UserAction};
use boincrs::ui::keymap::map_key_to_action;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[test]
fn focus_pane_cycles_all_columns() {
    let pane = FocusPane::Projects.next().next().next();
    assert_eq!(pane, FocusPane::Projects);
}

#[test]
fn focus_pane_cycles_backwards() {
    assert_eq!(FocusPane::Projects.previous(), FocusPane::Transfers);
    assert_eq!(FocusPane::Tasks.previous(), FocusPane::Projects);
}

#[test]
fn destructive_actions_require_confirmation() {
    assert!(UserAction::ProjectReset.is_destructive());
    assert!(UserAction::ProjectDetach.is_destructive());
    assert!(UserAction::TaskAbort.is_destructive());
    assert!(!UserAction::ProjectUpdate.is_destructive());
}

#[test]
fn keymap_supports_forward_and_reverse_focus_navigation() {
    let tab = KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE);
    let shift_tab = KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT);
    let left = KeyEvent::new(KeyCode::Left, KeyModifiers::NONE);
    let right = KeyEvent::new(KeyCode::Right, KeyModifiers::NONE);

    assert!(matches!(
        map_key_to_action(tab),
        Some(UserAction::CyclePane)
    ));
    assert!(matches!(
        map_key_to_action(shift_tab),
        Some(UserAction::PreviousPane)
    ));
    assert!(matches!(
        map_key_to_action(left),
        Some(UserAction::PreviousPane)
    ));
    assert!(matches!(
        map_key_to_action(right),
        Some(UserAction::CyclePane)
    ));
}

#[test]
fn escape_cancels_pending_confirmation() {
    let esc = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
    assert!(matches!(
        map_key_to_action(esc),
        Some(UserAction::CancelPending)
    ));
}
