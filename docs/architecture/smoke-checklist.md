# Local Smoke Checklist

1. Start BOINC client locally and ensure GUI RPC is enabled.
2. Run `BOINCRS_ENDPOINT=127.0.0.1:31416 BOINCRS_PASSWORD_FILE=/etc/boinc-client/gui_rpc_auth.cfg cargo run`.
3. Verify the TUI renders three panes: Projects, Tasks, Transfers.
4. Verify top `Selected Task` panel is visible and updates when task selection changes.
5. Press `r` and confirm status line reports refresh completion.
6. In Tasks pane, use `j/k` (or arrows) to move selection; confirm selected row marker changes.
7. Verify task grouping headings appear: `READY TO REPORT`, `RUNNING`, `WAITING / READY`.
8. Verify running tasks are ordered by completion percentage.
9. Trigger a safe project action (`u`, `s`, or `c`) and verify status updates.
10. Trigger a destructive action (`x`, `d`, or `b`) and verify confirmation prompt appears.
11. Press `n` to cancel and verify no action is sent.
12. Trigger again, press `y`, and verify action executes.
13. Press `q` to exit cleanly and restore terminal state.
