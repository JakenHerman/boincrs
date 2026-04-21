use ratatui::widgets::ListItem;

use crate::boinc::models::Project;

pub fn items(projects: &[Project]) -> Vec<ListItem<'_>> {
    projects
        .iter()
        .map(|p| {
            let line = format!(
                "{} ({}) [suspended:{} nmw:{}]",
                p.name, p.url, p.suspended_via_gui, p.dont_request_more_work
            );
            ListItem::new(line)
        })
        .collect()
}
