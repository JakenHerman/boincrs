use boincrs::app::actions::{FocusPane, UserAction};

#[test]
fn focus_pane_cycles_all_columns() {
    let pane = FocusPane::Projects.next().next().next();
    assert_eq!(pane, FocusPane::Projects);
}

#[test]
fn destructive_actions_require_confirmation() {
    assert!(UserAction::ProjectReset.is_destructive());
    assert!(UserAction::ProjectDetach.is_destructive());
    assert!(UserAction::TaskAbort.is_destructive());
    assert!(!UserAction::ProjectUpdate.is_destructive());
}
