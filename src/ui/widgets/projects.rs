use ratatui::widgets::ListItem;

use crate::boinc::models::Project;

pub fn items(projects: &[Project]) -> Vec<ListItem<'_>> {
    projects
        .iter()
        .map(|p| {
            let state = project_state_tags(p);
            let line = format!("{} {} ({})", p.name, state, p.url);
            ListItem::new(line)
        })
        .collect()
}

fn project_state_tags(project: &Project) -> String {
    let mut tags = Vec::new();
    if project.suspended_via_gui {
        tags.push("suspended");
    }
    if project.dont_request_more_work {
        tags.push("no-new-work");
    }
    if tags.is_empty() {
        tags.push("ready");
    }
    format!("[{}]", tags.join("] ["))
}
