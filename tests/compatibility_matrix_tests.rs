use boincrs::boinc::protocol::{
    parse_cc_status, parse_projects, parse_tasks, parse_transfers, ParsedTaskStatus,
};

#[test]
fn boinc_7_16_fixture_compatibility() {
    let projects = parse_projects(include_str!(
        "fixtures/compatibility/boinc_7_16/projects.xml"
    ))
    .expect("7.16 project fixture should parse");
    let tasks = parse_tasks(include_str!("fixtures/compatibility/boinc_7_16/tasks.xml"))
        .expect("7.16 task fixture should parse");
    let transfers = parse_transfers(include_str!(
        "fixtures/compatibility/boinc_7_16/transfers.xml"
    ))
    .expect("7.16 transfer fixture should parse");
    let status = parse_cc_status(include_str!("fixtures/compatibility/boinc_7_16/status.xml"))
        .expect("7.16 status fixture should parse");

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "Legacy Science");
    assert!(projects[0].dont_request_more_work);

    let running = tasks
        .iter()
        .find(|task| task.name == "legacy_active")
        .expect("expected legacy active task");
    assert!(running.active_task);
    assert_eq!(running.status, ParsedTaskStatus::Running);
    assert_eq!(running.application.as_deref(), Some("legacy_intelx86"));
    assert_eq!(running.checkpoint_cpu_time, Some(600.0));

    let report = tasks
        .iter()
        .find(|task| task.name == "legacy_report")
        .expect("expected legacy report task");
    assert_eq!(report.status, ParsedTaskStatus::ReadyToReport);
    assert_eq!(report.application.as_deref(), Some("app#701600"));
    assert_eq!(report.exit_status, Some(193));

    assert_eq!(transfers.len(), 1);
    assert_eq!(transfers[0].bytes_xferred, Some(2048));
    assert!(transfers[0].is_upload);
    assert_eq!(
        transfers[0].error_msg.as_deref(),
        Some("temporary dns failure")
    );

    assert_eq!(status.network_mode.as_deref(), Some("1"));
    assert_eq!(status.task_mode.as_deref(), Some("2"));
    assert_eq!(status.gpu_mode.as_deref(), Some("3"));
}

#[test]
fn boinc_7_20_fixture_compatibility() {
    let projects = parse_projects(include_str!(
        "fixtures/compatibility/boinc_7_20/projects.xml"
    ))
    .expect("7.20 project fixture should parse");
    let tasks = parse_tasks(include_str!("fixtures/compatibility/boinc_7_20/tasks.xml"))
        .expect("7.20 task fixture should parse");
    let transfers = parse_transfers(include_str!(
        "fixtures/compatibility/boinc_7_20/transfers.xml"
    ))
    .expect("7.20 transfer fixture should parse");
    let status = parse_cc_status(include_str!("fixtures/compatibility/boinc_7_20/status.xml"))
        .expect("7.20 status fixture should parse");

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "Bridge Science");
    assert!(projects[0].suspended_via_gui);

    let running = tasks
        .iter()
        .find(|task| task.name == "mid_running")
        .expect("expected 7.20 running task");
    assert!(running.active_task);
    assert_eq!(running.status, ParsedTaskStatus::Running);
    assert_eq!(running.application.as_deref(), Some("opencl_nvidia"));

    let waiting = tasks
        .iter()
        .find(|task| task.name == "mid_waiting")
        .expect("expected 7.20 waiting task");
    assert_eq!(waiting.status, ParsedTaskStatus::WaitingToRun);
    assert_eq!(waiting.application.as_deref(), Some("cuda102"));

    assert_eq!(transfers.len(), 1);
    assert_eq!(transfers[0].status, "active");
    assert_eq!(transfers[0].bytes_xferred, Some(4096));
    assert!(transfers[0].is_upload);
    assert_eq!(
        transfers[0].error_msg.as_deref(),
        Some("temporary http 500")
    );

    assert_eq!(status.network_mode.as_deref(), Some("1"));
    assert_eq!(status.task_mode.as_deref(), Some("2"));
    assert_eq!(status.gpu_mode.as_deref(), Some("3"));
}

#[test]
fn boinc_8_2_fixture_compatibility() {
    let projects = parse_projects(include_str!(
        "fixtures/compatibility/boinc_8_2/projects.xml"
    ))
    .expect("8.2 project fixture should parse");
    let tasks = parse_tasks(include_str!("fixtures/compatibility/boinc_8_2/tasks.xml"))
        .expect("8.2 task fixture should parse");
    let transfers = parse_transfers(include_str!(
        "fixtures/compatibility/boinc_8_2/transfers.xml"
    ))
    .expect("8.2 transfer fixture should parse");
    let status = parse_cc_status(include_str!("fixtures/compatibility/boinc_8_2/status.xml"))
        .expect("8.2 status fixture should parse");

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "Modern Science");
    assert!(!projects[0].dont_request_more_work);

    let running = tasks
        .iter()
        .find(|task| task.name == "modern_running")
        .expect("expected 8.2 running task");
    assert_eq!(running.status, ParsedTaskStatus::Running);
    assert_eq!(running.application.as_deref(), Some("vbox64_mt"));
    assert_eq!(running.received_time, Some(1_710_000_000.0));

    let waiting = tasks
        .iter()
        .find(|task| task.name == "modern_waiting")
        .expect("expected 8.2 waiting task");
    assert_eq!(waiting.status, ParsedTaskStatus::WaitingToRun);
    assert_eq!(waiting.application.as_deref(), Some("app#80209"));

    assert_eq!(transfers.len(), 1);
    assert_eq!(transfers[0].status, "downloading");
    assert_eq!(transfers[0].nbytes, Some(16_384));
    assert_eq!(transfers[0].bytes_xferred, Some(8_192));
    assert!(!transfers[0].is_upload);
    assert!(transfers[0].error_msg.is_none());

    assert_eq!(status.network_mode.as_deref(), Some("1"));
    assert_eq!(status.task_mode.as_deref(), Some("2"));
    assert_eq!(status.gpu_mode.as_deref(), Some("3"));
}
