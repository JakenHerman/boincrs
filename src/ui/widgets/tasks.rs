use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::ListItem;

use crate::boinc::models::{Task, TaskStatus};

pub fn items(tasks: &[Task]) -> Vec<ListItem<'_>> {
    let mut prev_group: Option<&'static str> = None;
    tasks
        .iter()
        .map(|t| {
            let progress = t
                .fraction_done
                .map(|v| (v * 100.0).clamp(0.0, 100.0))
                .map(|pct| format!("{pct:5.1}%"))
                .unwrap_or_else(|| "  n/a".to_string());
            let (icon, color) = status_visual(t.status);
            let name = truncate_with_ellipsis(t.name.as_str(), 36);
            let project = short_project(t.project_url.as_str());
            let group = group_heading(t.status);
            let mut spans = Vec::new();
            if prev_group != Some(group) {
                prev_group = Some(group);
                spans.push(Span::styled(
                    format!("--- {group} ---\n"),
                    Style::default().fg(Color::Blue),
                ));
            }
            spans.extend(vec![
                Span::raw(format!("{progress} ")),
                Span::styled(icon, Style::default().fg(color)),
                Span::raw(" "),
                Span::raw(name),
                Span::styled(
                    format!(" [{project}]"),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);
            ListItem::new(Line::from(spans))
        })
        .collect()
}

fn status_visual(status: TaskStatus) -> (&'static str, Color) {
    match status {
        TaskStatus::Running => ("🏃", Color::Green),
        TaskStatus::WaitingToRun => ("⏳", Color::Yellow),
        TaskStatus::ReadyToStart => ("🟢", Color::Cyan),
        TaskStatus::ReadyToReport => ("📤", Color::Magenta),
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
