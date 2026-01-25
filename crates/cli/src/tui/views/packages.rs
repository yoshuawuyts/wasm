use ratatui::{
    prelude::*,
    widgets::{Cell, Paragraph, Row, Table, Widget},
};
use wasm_package_manager::ImageEntry;

pub(crate) struct PackagesView<'a> {
    packages: &'a [ImageEntry],
}

impl<'a> PackagesView<'a> {
    pub(crate) fn new(packages: &'a [ImageEntry]) -> Self {
        Self { packages }
    }
}

impl Widget for PackagesView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.packages.is_empty() {
            Paragraph::new("No packages stored.")
                .centered()
                .render(area, buf);
        } else {
            // Create header row
            let header = Row::new(vec![
                Cell::from("Repository").style(Style::default().bold()),
                Cell::from("Registry").style(Style::default().bold()),
                Cell::from("Tag").style(Style::default().bold()),
                Cell::from("Digest").style(Style::default().bold()),
            ])
            .style(Style::default().fg(Color::Yellow));

            // Create data rows
            let rows: Vec<Row> = self
                .packages
                .iter()
                .map(|entry| {
                    let tag = entry.ref_tag.as_deref().unwrap_or("-");
                    let digest = entry
                        .ref_digest
                        .as_ref()
                        .map(|d| {
                            if d.len() > 16 {
                                format!("{}...", &d[..16])
                            } else {
                                d.clone()
                            }
                        })
                        .unwrap_or_else(|| "-".to_string());
                    Row::new(vec![
                        Cell::from(entry.ref_repository.clone()),
                        Cell::from(entry.ref_registry.clone()),
                        Cell::from(tag.to_string()),
                        Cell::from(digest),
                    ])
                })
                .collect();

            let table = Table::new(
                rows,
                [
                    Constraint::Percentage(35),
                    Constraint::Percentage(25),
                    Constraint::Percentage(15),
                    Constraint::Percentage(25),
                ],
            )
            .header(header)
            .row_highlight_style(Style::default().bg(Color::DarkGray));

            Widget::render(table, area, buf);
        }
    }
}
