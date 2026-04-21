use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListState, Paragraph};
use ratatui::Frame;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::app::actions::FocusPane;
use crate::app::state::{AppState, ConnectionState};
use crate::boinc::models::TaskStatus;
use crate::ui::widgets;

pub fn draw(frame: &mut Frame<'_>, state: &AppState) {
    let root = frame.area();
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(8),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(root);

    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(24),
            Constraint::Percentage(52),
            Constraint::Percentage(24),
        ])
        .split(vertical[1]);

    let filtered_tasks: Vec<_> = state.filtered_tasks().into_iter().cloned().collect();
    let filtered_transfers: Vec<_> = state.filtered_transfers().into_iter().cloned().collect();

    let projects = List::new(widgets::projects::items(&state.projects))
        .block(block("Projects", state.focus == FocusPane::Projects))
        .highlight_symbol("▶ ");
    let tasks = List::new(widgets::tasks::items(&filtered_tasks))
        .block(block("Tasks", state.focus == FocusPane::Tasks))
        .highlight_symbol("▶ ");
    let transfers = List::new(widgets::transfers::items(&filtered_transfers))
        .block(block("Transfers", state.focus == FocusPane::Transfers))
        .highlight_symbol("▶ ");

    let selected_task = Paragraph::new(selected_task_details(state)).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Selected Task"),
    );
    frame.render_widget(selected_task, vertical[0]);

    let mut project_state = ListState::default();
    if !state.projects.is_empty() {
        project_state.select(Some(state.selected_project_idx));
    }
    frame.render_stateful_widget(projects, panes[0], &mut project_state);

    let mut task_state = ListState::default();
    if !filtered_tasks.is_empty() {
        task_state.select(Some(state.selected_task_idx));
    }
    frame.render_stateful_widget(tasks, panes[1], &mut task_state);

    let mut transfer_state = ListState::default();
    if !filtered_transfers.is_empty() {
        transfer_state.select(Some(state.selected_transfer_idx));
    }
    frame.render_stateful_widget(transfers, panes[2], &mut transfer_state);

    let footer = Paragraph::new(
        "q quit | r refresh | tab pane | j/k scroll | y/n confirm | u/s/c/w/a/x/d project | t/g/b task | f transfer | 1-9 modes | D diag",
    )
    .block(Block::default().borders(Borders::ALL).title("Keymap"));
    frame.render_widget(footer, vertical[2]);

    let (status_text, status_style) = if let Some(pending) = &state.pending_confirmation {
        (
            format!("PENDING CONFIRMATION: {pending} (y/n)"),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        match &state.conn {
            ConnectionState::Connected => (state.status_line.clone(), Style::default()),
            ConnectionState::Retrying {
                attempt,
                delay_secs,
            } => (
                format!(
                    "[RETRYING] attempt {attempt} — reconnecting in {delay_secs}s. \
                     Press 'r' to retry now.",
                ),
                Style::default().fg(Color::Yellow),
            ),
            ConnectionState::TerminalError(_) => (
                format!("[ERROR] {} Press 'q' to quit.", state.status_line),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
        }
    };
    frame.render_widget(
        Paragraph::new(status_text)
            .style(status_style)
            .block(Block::default().borders(Borders::ALL).title("Status")),
        vertical[3],
    );
}

fn block(title: &str, focused: bool) -> Block<'_> {
    let style = if focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(style)
}

fn selected_task_details(state: &AppState) -> String {
    let Some(task) = state.selected_task_ref() else {
        return format!(
            "No task selected\nclient: run:{:?} net:{:?} gpu:{:?} msgs:{}",
            state.client_state.run_mode,
            state.client_state.network_mode,
            state.client_state.gpu_mode,
            state.client_state.messages.len()
        );
    };

    let progress = task
        .fraction_done
        .map(|v| format!("{:.1}%", (v * 100.0).clamp(0.0, 100.0)))
        .unwrap_or_else(|| "n/a".to_string());
    let status = task_status_label(task.status);
    let elapsed = format_duration(task.elapsed_seconds);
    let remaining = format_duration(task.remaining_seconds);
    let deadline = format_deadline(task.report_deadline);
    let application = task
        .application
        .clone()
        .unwrap_or_else(|| "n/a".to_string());
    let checkpoint = format_duration(task.checkpoint_cpu_time);
    let exit_info = match task.exit_status {
        Some(code) => format!(" | exit:{code}"),
        None => String::new(),
    };

    let line1 = format!(
        "name:{} | project:{} | progress:{} | status:{}{}",
        task.name,
        short_project(task.project_url.as_str()),
        progress,
        status,
        exit_info,
    );
    let line2 = format!(
        "elapsed:{} | chkpt:{} | remaining:{} | deadline:{} | app:{} | run:{:?} net:{:?} gpu:{:?}",
        elapsed,
        checkpoint,
        remaining,
        deadline,
        application,
        state.client_state.run_mode,
        state.client_state.network_mode,
        state.client_state.gpu_mode,
    );
    format!("{line1}\n{line2}")
}

fn task_status_label(status: TaskStatus) -> &'static str {
    match status {
        TaskStatus::Running => "running",
        TaskStatus::WaitingToRun => "waiting-to-run",
        TaskStatus::ReadyToStart => "ready-to-start",
        TaskStatus::ReadyToReport => "ready-to-report",
    }
}

fn format_duration(seconds: Option<f64>) -> String {
    let Some(raw) = seconds else {
        return "n/a".to_string();
    };
    let total = if raw.is_sign_negative() {
        0
    } else {
        raw as u64
    };
    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;
    format!("{h:02}:{m:02}:{s:02}")
}

fn format_deadline(deadline_epoch: Option<f64>) -> String {
    let Some(raw) = deadline_epoch else {
        return "n/a".to_string();
    };
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(v) => v.as_secs_f64(),
        Err(_) => return format!("epoch:{raw:.0}"),
    };
    let delta = raw - now;
    if delta >= 0.0 {
        let total = delta as u64;
        let days = total / 86_400;
        let hours = (total % 86_400) / 3_600;
        format!("in {days}d {hours}h")
    } else {
        "past-due".to_string()
    }
}

fn short_project(url: &str) -> String {
    let trimmed = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/');
    trimmed.split('/').next().unwrap_or(trimmed).to_string()
}
