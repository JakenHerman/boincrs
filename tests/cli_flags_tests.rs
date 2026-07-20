//! End-to-end checks for the `--help` / `--version` command-line flags.
//!
//! These exercise the real compiled binary (no BOINC daemon required), which is
//! also what the Homebrew `test do` block and the Chocolatey package smoke test
//! rely on after installation.

use std::process::Command;

fn boincrs() -> Command {
    Command::new(env!("CARGO_BIN_EXE_boincrs"))
}

#[test]
fn version_flag_prints_version_and_exits_zero() {
    let output = boincrs()
        .arg("--version")
        .output()
        .expect("run boincrs --version");

    assert!(output.status.success(), "exit status: {:?}", output.status);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("boincrs"), "stdout was: {stdout:?}");
    assert!(
        stdout.contains(env!("CARGO_PKG_VERSION")),
        "stdout {stdout:?} did not contain version {}",
        env!("CARGO_PKG_VERSION"),
    );
}

#[test]
fn short_version_flag_matches_long_flag() {
    let output = boincrs().arg("-V").output().expect("run boincrs -V");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn help_flag_prints_usage_and_exits_zero() {
    let output = boincrs()
        .arg("--help")
        .output()
        .expect("run boincrs --help");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"), "stdout was: {stdout:?}");
    assert!(stdout.contains("--version"), "stdout was: {stdout:?}");
}

#[test]
fn unrecognized_argument_errors_with_exit_code_two() {
    let output = boincrs()
        .arg("--definitely-not-a-flag")
        .output()
        .expect("run boincrs with a bad flag");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unrecognized argument"),
        "stderr was: {stderr:?}",
    );
}
