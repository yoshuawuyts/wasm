use ratatui::{
    prelude::*,
    widgets::{Cell, Paragraph, Row, Table, Widget},
};
use wasm_package_manager::StateInfo;

/// View for displaying settings
pub struct SettingsView<'a> {
    state_info: Option<&'a StateInfo>,
}

impl<'a> std::fmt::Debug for SettingsView<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettingsView")
            .field("has_state_info", &self.state_info.is_some())
            .finish()
    }
}

impl<'a> SettingsView<'a> {
    /// Create a new settings view
    #[must_use]
    pub fn new(state_info: Option<&'a StateInfo>) -> Self {
        Self { state_info }
    }
}

impl Widget for SettingsView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.state_info {
            Some(info) => {
                // Split area for migrations section and storage table
                let layout =
                    Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(area);

                // Migrations section
                let migrations = Text::from(vec![
                    Line::from(vec![Span::styled(
                        "Migrations",
                        Style::default().bold().fg(Color::Yellow),
                    )]),
                    Line::from(format!(
                        "  Current:  {}/{}",
                        info.migration_current(),
                        info.migration_total()
                    )),
                ]);
                if let Some(migrations_area) = layout.first() {
                    Paragraph::new(migrations).render(*migrations_area, buf);
                }

                // Storage section with table
                if let Some(storage_outer_area) = layout.get(1) {
                    let storage_layout =
                        Layout::vertical([Constraint::Length(1), Constraint::Min(0)])
                            .split(*storage_outer_area);

                    let storage_header = Line::from(vec![Span::styled(
                        "Storage",
                        Style::default().bold().fg(Color::Yellow),
                    )]);
                    if let Some(storage_header_area) = storage_layout.first() {
                        Paragraph::new(storage_header).render(*storage_header_area, buf);
                    }

                    // Compute column widths based on content
                    let executable_path = info.executable().display().to_string();
                    let data_dir_path = info.data_dir().display().to_string();
                    let layers_dir_path = info.layers_dir().display().to_string();
                    let metadata_file_path = info.metadata_file().display().to_string();
                    let layers_size = super::format_size(info.layers_size());
                    let metadata_size = super::format_size(info.metadata_size());

                    // Column 1: longest is "Image metadata" = 14 chars
                    let col1_width = 14;
                    // Column 2: longest path
                    let col2_width = executable_path
                        .len()
                        .max(data_dir_path.len())
                        .max(layers_dir_path.len())
                        .max(metadata_file_path.len());
                    // Column 3: longest size string or "-"
                    let col3_width = layers_size.len().max(metadata_size.len()).max(1);

                    // Create data rows
                    let rows = vec![
                        Row::new(vec![
                            Cell::from("Executable"),
                            Cell::from(executable_path),
                            Cell::from("-"),
                        ]),
                        Row::new(vec![
                            Cell::from("Data storage"),
                            Cell::from(data_dir_path),
                            Cell::from("-"),
                        ]),
                        Row::new(vec![
                            Cell::from("Image layers"),
                            Cell::from(layers_dir_path),
                            Cell::from(layers_size),
                        ]),
                        Row::new(vec![
                            Cell::from("Image metadata"),
                            Cell::from(metadata_file_path),
                            Cell::from(metadata_size),
                        ]),
                    ];

                    let table = Table::new(
                        rows,
                        [
                            Constraint::Length(col1_width as u16),
                            Constraint::Length(col2_width as u16),
                            Constraint::Length(col3_width as u16),
                        ],
                    )
                    .column_spacing(3);

                    if let Some(storage_table_area) = storage_layout.get(1) {
                        Widget::render(table, *storage_table_area, buf);
                    }
                }
            }
            None => {
                Paragraph::new("Loading state information...").render(area, buf);
            }
        }
    }
}
