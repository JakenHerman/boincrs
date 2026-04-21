use crate::error::{AppError, AppResult};
use quick_xml::de::from_str;
use serde::Deserialize;

const EOM: u8 = 0x03;

/// Wraps a BOINC inner command in the GUI RPC request envelope.
///
/// Appends BOINC end-of-message marker (`0x03`) required by the wire protocol.
///
/// # Examples
///
/// ```
/// use boincrs::boinc::protocol::frame_request;
///
/// let bytes = frame_request("<get_cc_status/>");
/// assert_eq!(bytes.last().copied(), Some(0x03));
/// ```
pub fn frame_request(inner_xml: &str) -> Vec<u8> {
    let request = format!("<boinc_gui_rpc_request>{inner_xml}</boinc_gui_rpc_request>");
    let mut bytes = request.into_bytes();
    bytes.push(EOM);
    bytes
}

/// Parses a raw BOINC reply payload into UTF-8 XML text.
///
/// Accepts payloads with or without trailing EOM marker.
pub fn parse_response_payload(raw: &[u8]) -> AppResult<String> {
    let trimmed = match raw.last() {
        Some(last) if *last == EOM => &raw[..raw.len().saturating_sub(1)],
        _ => raw,
    };
    let text = String::from_utf8(trimmed.to_vec())
        .map_err(|_| AppError::InvalidResponse("response is not valid UTF-8".to_string()))?;
    if !text.contains("<boinc_gui_rpc_reply>") {
        return Err(AppError::InvalidResponse(
            "missing <boinc_gui_rpc_reply> root".to_string(),
        ));
    }
    Ok(text)
}

#[derive(Debug, Deserialize)]
struct Auth1Reply {
    nonce: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Auth2Reply {
    authorized: Option<String>,
}

/// Parsed project shape used by read API mapping.
#[derive(Debug)]
pub struct ParsedProject {
    pub url: String,
    pub name: String,
    pub suspended_via_gui: bool,
    pub dont_request_more_work: bool,
}

/// Parsed task/result shape used by read API mapping.
#[derive(Debug)]
pub struct ParsedTask {
    pub project_url: String,
    pub name: String,
    pub active_task: bool,
    pub suspended_via_gui: bool,
    pub fraction_done: Option<f64>,
    pub status: ParsedTaskStatus,
    pub elapsed_seconds: Option<f64>,
    pub remaining_seconds: Option<f64>,
    pub report_deadline: Option<f64>,
    pub application: Option<String>,
}

/// Parsed task status used before mapping to UI-facing `TaskStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsedTaskStatus {
    Running,
    WaitingToRun,
    ReadyToStart,
    ReadyToReport,
}

/// Parsed transfer shape used by read API mapping.
#[derive(Debug)]
pub struct ParsedTransfer {
    pub project_url: String,
    pub file_name: String,
    pub status: String,
    pub nbytes: Option<u64>,
    pub bytes_xferred: Option<u64>,
    pub xfer_speed: Option<f64>,
    pub is_upload: bool,
    pub error_msg: Option<String>,
}

/// Parsed BOINC client mode values.
#[derive(Debug)]
pub struct ParsedCcStatus {
    pub network_mode: Option<String>,
    pub task_mode: Option<String>,
    pub gpu_mode: Option<String>,
}

/// Extracts nonce from BOINC `auth1` reply.
pub fn parse_auth_nonce(xml: &str) -> AppResult<String> {
    if let Ok(reply) = from_str::<Auth1Reply>(xml) {
        if let Some(nonce) = reply.nonce {
            return Ok(nonce);
        }
    }
    extract_tag_value(xml, "nonce")
        .ok_or_else(|| AppError::InvalidResponse("missing auth nonce".to_string()))
}

/// Computes BOINC nonce hash (`md5(nonce + password)`).
///
/// # Examples
///
/// ```
/// use boincrs::boinc::protocol::compute_nonce_hash;
///
/// assert_eq!(
///     compute_nonce_hash("nonce", "password"),
///     "9ec96ea2f1ac5aa36b73ae4b5f9f081d"
/// );
/// ```
pub fn compute_nonce_hash(nonce: &str, password: &str) -> String {
    format!("{:x}", md5::compute(format!("{nonce}{password}")))
}

/// Parses authorization status from BOINC `auth2` reply.
pub fn parse_auth_authorized(xml: &str) -> AppResult<bool> {
    if let Ok(reply) = from_str::<Auth2Reply>(xml) {
        if let Some(value) = reply.authorized {
            return Ok(value == "1");
        }
    }
    Ok(extract_tag_value(xml, "authorized").as_deref() == Some("1"))
}

/// Parses project list from `get_project_status` reply.
pub fn parse_projects(xml: &str) -> AppResult<Vec<ParsedProject>> {
    let mut out = Vec::new();
    for block in extract_block_items(xml, "project") {
        out.push(ParsedProject {
            url: extract_tag_value(&block, "master_url").unwrap_or_default(),
            name: extract_tag_value(&block, "project_name").unwrap_or_default(),
            suspended_via_gui: parse_bool_tag(&block, "suspended_via_gui"),
            dont_request_more_work: parse_bool_tag(&block, "dont_request_more_work"),
        });
    }
    Ok(out)
}

/// Parses task/result list from `get_results` reply.
pub fn parse_tasks(xml: &str) -> AppResult<Vec<ParsedTask>> {
    let mut out = Vec::new();
    for block in extract_block_items(xml, "result") {
        let active_task = parse_bool_tag(&block, "active_task")
            || extract_tag_value(&block, "active_task_state")
                .and_then(|v| v.trim().parse::<i32>().ok())
                .map(|v| v > 0)
                .unwrap_or(false);
        let status = classify_task_status(&block, active_task);
        out.push(ParsedTask {
            project_url: extract_tag_value(&block, "project_url").unwrap_or_default(),
            name: extract_tag_value(&block, "name").unwrap_or_default(),
            active_task,
            suspended_via_gui: parse_bool_tag(&block, "suspended_via_gui"),
            fraction_done: parse_f64_tag(&block, "fraction_done"),
            status,
            elapsed_seconds: parse_f64_tag(&block, "elapsed_time"),
            remaining_seconds: parse_f64_tag(&block, "estimated_cpu_time_remaining"),
            report_deadline: parse_f64_tag(&block, "report_deadline"),
            application: extract_application(&block),
        });
    }
    Ok(out)
}

/// Parses transfer list from `get_file_transfers` reply.
pub fn parse_transfers(xml: &str) -> AppResult<Vec<ParsedTransfer>> {
    let mut out = Vec::new();
    for block in extract_block_items(xml, "file_transfer") {
        let status = extract_tag_value(&block, "status").unwrap_or_else(|| "unknown".to_string());
        let is_upload = parse_bool_tag(&block, "generated_locally")
            || extract_tag_value(&block, "is_upload")
                .map(|v| v.trim() == "1")
                .unwrap_or(false);
        let bytes_xferred = extract_tag_value(&block, "bytes_xferred")
            .or_else(|| extract_tag_value(&block, "last_bytes_xferred"))
            .and_then(|v| v.trim().parse::<u64>().ok());
        let error_msg = extract_tag_value(&block, "error_msg")
            .or_else(|| extract_tag_value(&block, "error"))
            .filter(|s| !s.trim().is_empty());
        out.push(ParsedTransfer {
            project_url: extract_tag_value(&block, "project_url").unwrap_or_default(),
            file_name: extract_tag_value(&block, "name").unwrap_or_default(),
            status: if status.is_empty() {
                "unknown".to_string()
            } else {
                status
            },
            nbytes: extract_tag_value(&block, "nbytes")
                .and_then(|v| v.trim().parse::<f64>().ok())
                .map(|v| v as u64),
            bytes_xferred,
            xfer_speed: parse_f64_tag(&block, "xfer_speed"),
            is_upload,
            error_msg,
        });
    }
    Ok(out)
}

/// Parses mode values from `get_cc_status` reply.
pub fn parse_cc_status(xml: &str) -> AppResult<ParsedCcStatus> {
    Ok(ParsedCcStatus {
        network_mode: parse_mode_value(xml, "network_mode"),
        task_mode: parse_mode_value(xml, "task_mode"),
        gpu_mode: parse_mode_value(xml, "gpu_mode"),
    })
}

/// Returns `true` when a reply indicates authorization failure.
pub fn reply_has_unauthorized(xml: &str) -> bool {
    xml.contains("<unauthorized/>") || xml.contains("<unauthorized>")
}

fn parse_mode_value(xml: &str, tag: &str) -> Option<String> {
    let inner = extract_tag_value(xml, tag)?;
    extract_tag_value(&inner, "perm_mode")
        .or_else(|| extract_tag_value(&inner, "mode"))
        .or_else(|| {
            let trimmed = inner.trim();
            if trimmed.is_empty() || trimmed.starts_with('<') {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
}

fn extract_tag_value(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = xml.find(&open)?;
    let value_start = start + open.len();
    let end_rel = xml[value_start..].find(&close)?;
    Some(xml[value_start..value_start + end_rel].to_string())
}

fn parse_bool_tag(xml: &str, tag: &str) -> bool {
    if has_self_closing_tag(xml, tag) {
        return true;
    }
    extract_tag_value(xml, tag)
        .map(|v| {
            let t = v.trim();
            t == "1" || t.eq_ignore_ascii_case("true")
        })
        .unwrap_or(false)
}

fn parse_f64_tag(xml: &str, tag: &str) -> Option<f64> {
    extract_tag_value(xml, tag).and_then(|v| v.trim().parse::<f64>().ok())
}

fn extract_application(block: &str) -> Option<String> {
    extract_tag_value(block, "resources")
        .or_else(|| extract_tag_value(block, "plan_class"))
        .or_else(|| extract_tag_value(block, "app_version_num").map(|v| format!("app#{v}")))
}

fn classify_task_status(block: &str, active_task: bool) -> ParsedTaskStatus {
    if parse_bool_tag(block, "ready_to_report") {
        return ParsedTaskStatus::ReadyToReport;
    }
    if active_task {
        return ParsedTaskStatus::Running;
    }

    // BOINC scheduler_state values differ by version; treat "1" and explicit
    // waiting markers as queued/preempted tasks.
    let scheduler_state = extract_tag_value(block, "scheduler_state")
        .unwrap_or_default()
        .to_lowercase();
    if scheduler_state == "1"
        || scheduler_state.contains("wait")
        || scheduler_state.contains("preempt")
    {
        return ParsedTaskStatus::WaitingToRun;
    }

    ParsedTaskStatus::ReadyToStart
}

fn extract_block_items(xml: &str, tag: &str) -> Vec<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut cursor = 0usize;
    let mut out = Vec::new();
    while let Some(start_rel) = xml[cursor..].find(&open) {
        let start = cursor + start_rel + open.len();
        if let Some(end_rel) = xml[start..].find(&close) {
            let end = start + end_rel;
            out.push(xml[start..end].to_string());
            cursor = end + close.len();
        } else {
            break;
        }
    }
    out
}

fn has_self_closing_tag(xml: &str, tag: &str) -> bool {
    let compact = xml.replace(char::is_whitespace, "");
    let needle1 = format!("<{tag}/>");
    let needle2 = format!("<{tag}></{tag}>");
    compact.contains(&needle1) || compact.contains(&needle2)
}
