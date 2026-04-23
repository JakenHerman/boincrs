use ratatui::style::{Color, Modifier, Style};

use crate::boinc::models::TaskStatus;

#[derive(Debug, Clone, Copy)]
pub struct UiTheme {
    no_color: bool,
}

impl UiTheme {
    pub fn active() -> Self {
        Self {
            no_color: std::env::var_os("NO_COLOR").is_some(),
        }
    }

    pub fn pane_style(self, focused: bool) -> Style {
        let mut style = Style::default();
        if focused {
            style = style.add_modifier(Modifier::BOLD);
            if !self.no_color {
                style = style.fg(Color::Cyan);
            }
        }
        style
    }

    pub fn selected_item_style(self) -> Style {
        Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD)
    }

    pub fn group_heading_style(self) -> Style {
        let mut style = Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
        if !self.no_color {
            style = style.fg(Color::Cyan);
        }
        style
    }

    pub fn task_status_style(self, status: TaskStatus) -> Style {
        let style = Style::default().add_modifier(Modifier::BOLD);
        if self.no_color {
            return style;
        }

        let color = match status {
            TaskStatus::Running => Color::Green,
            TaskStatus::WaitingToRun => Color::Yellow,
            TaskStatus::ReadyToStart => Color::Blue,
            TaskStatus::ReadyToReport => Color::Magenta,
        };
        style.fg(color)
    }

    pub fn info_style(self) -> Style {
        Style::default()
    }

    pub fn warning_style(self) -> Style {
        let mut style = Style::default().add_modifier(Modifier::BOLD);
        if !self.no_color {
            style = style.fg(Color::Yellow);
        }
        style
    }

    pub fn error_style(self) -> Style {
        let mut style = Style::default().add_modifier(Modifier::BOLD);
        if !self.no_color {
            style = style.fg(Color::Red);
        }
        style
    }
}
