use ratatui::{
    prelude::*,
    widgets::{Cell, Paragraph, Row, StatefulWidget, Table, TableState, Widget},
};
use wasm_package_manager::ImageEntry;

/// State for the packages list view
#[derive(Debug, Default)]
pub(crate) struct PackagesViewState {
    pub table_state: TableState,
}

impl PackagesViewState {
    pub(crate) fn new() -> Self {
        Self {
            table_state: TableState::default().with_selected(Some(0)),
        }
    }

    pub(crate) fn selected(&self) -> Option<usize> {
        self.table_state.selected()
    }

    pub(crate) fn select_next(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        let current = self.table_state.selected().unwrap_or(0);
        let next = if current >= len - 1 { 0 } else { current + 1 };
        self.table_state.select(Some(next));
    }

    pub(crate) fn select_prev(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        let current = self.table_state.selected().unwrap_or(0);
        let prev = if current == 0 { len - 1 } else { current - 1 };
        self.table_state.select(Some(prev));
    }
}

pub(crate) struct PackagesView<'a> {
    packages: &'a [ImageEntry],
}

impl<'a> PackagesView<'a> {
    pub(crate) fn new(packages: &'a [ImageEntry]) -> Self {
        Self { packages }
    }
}

impl StatefulWidget for PackagesView<'_> {
    type State = PackagesViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Split area into content and shortcuts bar
        let layout = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(area);
        let content_area = layout[0];
        let shortcuts_area = layout[1];

        if self.packages.is_empty() {
            Paragraph::new("No packages stored.")
                .centered()
                .render(content_area, buf);
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

            StatefulWidget::render(table, content_area, buf, &mut state.table_state);
        }

        // Render shortcuts bar
        let shortcuts = Line::from(vec![
            Span::styled(" p ", Style::default().fg(Color::Black).bg(Color::Yellow)),
            Span::raw(" Pull  "),
            Span::styled(" d ", Style::default().fg(Color::Black).bg(Color::Yellow)),
            Span::raw(" Delete  "),
            Span::styled(
                " Enter ",
                Style::default().fg(Color::Black).bg(Color::Yellow),
            ),
            Span::raw(" View details "),
        ]);
        Paragraph::new(shortcuts)
            .style(Style::default().fg(Color::DarkGray))
            .render(shortcuts_area, buf);
    }
}

impl Widget for PackagesView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = PackagesViewState::new();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}
