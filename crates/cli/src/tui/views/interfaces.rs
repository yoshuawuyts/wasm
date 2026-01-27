use ratatui::{
    prelude::*,
    widgets::{Cell, Paragraph, Row, StatefulWidget, Table, TableState, Widget},
};
use wasm_detector::{InterfaceInfo, InterfaceKind, WasmEntry};

/// State for the interfaces list view
#[derive(Debug, Default)]
pub struct InterfacesViewState {
    /// Table selection state
    pub table_state: TableState,
}

impl InterfacesViewState {
    /// Creates a new interfaces view state
    #[must_use]
    pub fn new() -> Self {
        Self {
            table_state: TableState::default().with_selected(Some(0)),
        }
    }

    #[allow(dead_code)] // Will be used when interface detail view is implemented
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

/// View for the Interfaces tab
#[derive(Debug)]
pub struct InterfacesView<'a> {
    wasm_files: &'a [WasmEntry],
}

impl<'a> InterfacesView<'a> {
    /// Create a new InterfacesView with the given WASM files
    #[must_use]
    pub fn new(wasm_files: &'a [WasmEntry]) -> Self {
        Self { wasm_files }
    }
}

impl StatefulWidget for InterfacesView<'_> {
    type State = InterfacesViewState;

    #[allow(clippy::indexing_slicing)]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Collect all unique interfaces from all WASM files
        let mut all_interfaces: Vec<(&InterfaceInfo, &WasmEntry)> = Vec::new();

        for wasm_entry in self.wasm_files {
            for interface in wasm_entry.interfaces() {
                all_interfaces.push((interface, wasm_entry));
            }
        }

        // Split area into content and shortcuts bar
        let layout = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(area);
        let content_area = layout[0];
        let shortcuts_area = layout[1];

        if all_interfaces.is_empty() {
            let message = Paragraph::new(
                "No WIT interfaces detected in local WASM files.\n\nPress 'r' to refresh.",
            )
            .centered()
            .style(Style::default().fg(Color::DarkGray));
            message.render(content_area, buf);
        } else {
            // Create header row
            let header = Row::new(vec![
                Cell::from("Interface").style(Style::default().bold()),
                Cell::from("Type").style(Style::default().bold()),
                Cell::from("Version").style(Style::default().bold()),
                Cell::from("Source File").style(Style::default().bold()),
            ])
            .style(Style::default().fg(Color::Yellow));

            // Create data rows
            let rows: Vec<Row> = all_interfaces
                .iter()
                .map(|(interface, wasm_entry)| {
                    let kind_str = match interface.kind {
                        InterfaceKind::Dependency => "Dependency",
                        InterfaceKind::ChildComponent => "Component",
                        InterfaceKind::ChildModule => "Module",
                    };

                    let version_str = interface
                        .version
                        .as_ref()
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string());

                    let file_name = wasm_entry.file_name().unwrap_or("<unnamed>");

                    Row::new(vec![
                        Cell::from(interface.name.clone()),
                        Cell::from(kind_str),
                        Cell::from(version_str),
                        Cell::from(file_name.to_string()),
                    ])
                })
                .collect();

            let table = Table::new(
                rows,
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(30),
                ],
            )
            .header(header)
            .row_highlight_style(Style::default().bg(Color::DarkGray));

            StatefulWidget::render(table, content_area, buf, &mut state.table_state);
        }

        // Render shortcuts bar
        let shortcuts = Line::from(vec![
            Span::styled(" ↑↓ ", Style::default().fg(Color::Black).bg(Color::Yellow)),
            Span::raw(" Navigate  "),
            Span::styled(
                " Enter ",
                Style::default().fg(Color::Black).bg(Color::Yellow),
            ),
            Span::raw(" View details  "),
            Span::styled(" r ", Style::default().fg(Color::Black).bg(Color::Yellow)),
            Span::raw(" Refresh "),
        ]);
        Paragraph::new(shortcuts)
            .style(Style::default().fg(Color::DarkGray))
            .render(shortcuts_area, buf);
    }
}

impl Widget for InterfacesView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = InterfacesViewState::new();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}
