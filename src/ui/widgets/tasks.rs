use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::ListItem;

use crate::boinc::models::{Task, TaskStatus};
use crate::ui::theme::UiTheme;

pub fn items(tasks: &[Task]) -> Vec<ListItem<'_>> {
    let mut prev_group: Option<&'static str> = None;
    let theme = UiTheme::active();
    tasks
        .iter()
        .map(|t| {
            let progress = t
                .fraction_done
                .map(|v| (v * 100.0).clamp(0.0, 100.0))
                .map(|pct| format!("{pct:5.1}%"))
                .unwrap_or_else(|| "  n/a".to_string());
            let name = truncate_with_ellipsis(t.name.as_str(), 36);
            let project = short_project(t.project_url.as_str());
            let group = group_heading(t.status);
            let mut spans = Vec::new();
            if prev_group != Some(group) {
                prev_group = Some(group);
                spans.push(Span::styled(
                    format!("--- {group} ---\n"),
                    theme.group_heading_style(),
                ));
            }
            spans.push(Span::raw(format!("{progress} ")));
            spans.push(Span::styled(
                status_tag(t.status),
                theme.task_status_style(t.status),
            ));
            if t.active_task {
                spans.push(Span::raw(" [active]"));
            }
            if t.suspended_via_gui {
                spans.push(Span::raw(" [paused]"));
            }
            if let Some(code) = t.exit_status {
                if code != 0 {
                    spans.push(Span::raw(format!(" [exit:{code}]")));
                }
            }
            spans.push(Span::raw(" "));
            spans.push(Span::raw(name));
            spans.push(Span::styled(format!(" ({project})"), Style::default()));
            ListItem::new(Line::from(spans))
        })
        .collect()
}

fn status_tag(status: TaskStatus) -> &'static str {
    match status {
        TaskStatus::Running => "[RUN]",
        TaskStatus::WaitingToRun => "[WAIT]",
        TaskStatus::ReadyToStart => "[READY]",
        TaskStatus::ReadyToReport => "[REPORT]",
    }
}

fn group_heading(status: TaskStatus) -> &'static str {
    match status {
        TaskStatus::ReadyToReport => "READY TO REPORT",
        TaskStatus::Running => "RUNNING",
        TaskStatus::WaitingToRun | TaskStatus::ReadyToStart => "WAITING / READY",
    }
}

fn truncate_with_ellipsis(input: &str, max_chars: usize) -> String {
    let count = input.chars().count();
    if count <= max_chars {
        return input.to_string();
    }
    if max_chars <= 1 {
        return "…".to_string();
    }
    let kept: String = input.chars().take(max_chars - 1).collect();
    format!("{kept}…")
}

fn short_project(url: &str) -> String {
    let trimmed = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/');
    trimmed.split('/').next().unwrap_or(trimmed).to_string()
}
