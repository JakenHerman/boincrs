use ratatui::widgets::ListItem;

use crate::boinc::models::Transfer;

pub fn items(transfers: &[Transfer]) -> Vec<ListItem<'_>> {
    transfers.iter().map(render_transfer).collect()
}

fn render_transfer(t: &Transfer) -> ListItem<'_> {
    let direction = if t.is_upload { "↑" } else { "↓" };
    let progress = match (t.bytes_xferred, t.nbytes) {
        (Some(xferred), Some(total)) if total > 0 => {
            let pct = (xferred as f64 / total as f64 * 100.0).clamp(0.0, 100.0);
            format!("{:.0}% ({}/{})", pct, fmt_bytes(xferred), fmt_bytes(total))
        }
        (Some(xferred), _) => fmt_bytes(xferred),
        _ => "pending".to_string(),
    };
    let speed = t
        .xfer_speed
        .map(|s| format!(" @ {}/s", fmt_bytes(s as u64)))
        .unwrap_or_default();
    let error = t
        .error_msg
        .as_deref()
        .map(|e| format!(" [err: {e}]"))
        .unwrap_or_default();
    ListItem::new(format!(
        "{direction} {} | {progress}{speed}{error}",
        t.file_name,
    ))
}

fn fmt_bytes(n: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;
    if n >= GB {
        format!("{:.1}GB", n as f64 / GB as f64)
    } else if n >= MB {
        format!("{:.1}MB", n as f64 / MB as f64)
    } else if n >= KB {
        format!("{:.1}KB", n as f64 / KB as f64)
    } else {
        format!("{n}B")
    }
}
