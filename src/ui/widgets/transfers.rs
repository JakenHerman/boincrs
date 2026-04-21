use ratatui::widgets::ListItem;

use crate::boinc::models::Transfer;

pub fn items(transfers: &[Transfer]) -> Vec<ListItem<'_>> {
    transfers
        .iter()
        .map(|t| ListItem::new(format!("{} [{}] {}", t.file_name, t.project_url, t.status)))
        .collect()
}
