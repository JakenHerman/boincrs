use boincrs::boinc::protocol::{
    frame_request, parse_auth_authorized, parse_auth_nonce, parse_cc_status, parse_projects,
    parse_response_payload, parse_tasks, parse_transfers, reply_has_unauthorized, ParsedTaskStatus,
};

#[test]
fn frames_request_with_root_and_eom() {
    let framed = frame_request("<get_cc_status/>");
    let as_text =
        String::from_utf8(framed[..framed.len() - 1].to_vec()).expect("request should be utf-8");
    assert!(as_text.contains("<boinc_gui_rpc_request>"));
    assert_eq!(framed.last().copied(), Some(0x03));
}

#[test]
fn parses_reply_payload() {
    let payload = b"<boinc_gui_rpc_reply><success/></boinc_gui_rpc_reply>\x03";
    let parsed = parse_response_payload(payload).expect("should parse valid reply");
    assert!(parsed.contains("<success/>"));
}

#[test]
fn parses_auth_responses() {
    let nonce_xml = "<boinc_gui_rpc_reply><nonce>abc123</nonce></boinc_gui_rpc_reply>";
    let auth_xml = "<boinc_gui_rpc_reply><authorized>1</authorized></boinc_gui_rpc_reply>";
    let nonce = parse_auth_nonce(nonce_xml).expect("nonce should parse");
    let authorized = parse_auth_authorized(auth_xml).expect("authorized should parse");
    assert_eq!(nonce, "abc123");
    assert!(authorized);
}

#[test]
fn detects_unauthorized_reply() {
    let xml = "<boinc_gui_rpc_reply><unauthorized/></boinc_gui_rpc_reply>";
    assert!(reply_has_unauthorized(xml));
}

#[test]
fn parses_project_task_transfer_and_status_replies() {
    let projects_xml = "<boinc_gui_rpc_reply><project><master_url>u</master_url><project_name>n</project_name><suspended_via_gui>1</suspended_via_gui><dont_request_more_work>0</dont_request_more_work></project></boinc_gui_rpc_reply>";
    let tasks_xml = "<boinc_gui_rpc_reply><result><project_url>u</project_url><name>t</name><active_task>1</active_task><suspended_via_gui>0</suspended_via_gui></result><result><project_url>u</project_url><name>done</name><ready_to_report/></result></boinc_gui_rpc_reply>";
    let transfers_xml = "<boinc_gui_rpc_reply><file_transfer><project_url>u</project_url><name>f</name><status>ok</status></file_transfer></boinc_gui_rpc_reply>";
    let status_xml = "<boinc_gui_rpc_reply><network_mode><perm_mode>1</perm_mode></network_mode><task_mode>2</task_mode><gpu_mode><perm_mode>3</perm_mode></gpu_mode></boinc_gui_rpc_reply>";

    let projects = parse_projects(projects_xml).expect("projects should parse");
    let tasks = parse_tasks(tasks_xml).expect("tasks should parse");
    let transfers = parse_transfers(transfers_xml).expect("transfers should parse");
    let status = parse_cc_status(status_xml).expect("status should parse");

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "n");
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].name, "t");
    assert_eq!(tasks[0].status, ParsedTaskStatus::Running);
    assert_eq!(tasks[1].name, "done");
    assert_eq!(tasks[1].status, ParsedTaskStatus::ReadyToReport);
    assert_eq!(transfers.len(), 1);
    assert_eq!(transfers[0].file_name, "f");
    assert_eq!(status.network_mode.as_deref(), Some("1"));
    assert_eq!(status.task_mode.as_deref(), Some("2"));
    assert_eq!(status.gpu_mode.as_deref(), Some("3"));
}
