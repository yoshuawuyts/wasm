use ratatui::{
    prelude::*,
    widgets::{Paragraph, Widget},
};
use wasm_package_manager::StateInfo;

pub(crate) struct SettingsView<'a> {
    state_info: Option<&'a StateInfo>,
}

impl<'a> SettingsView<'a> {
    pub(crate) fn new(state_info: Option<&'a StateInfo>) -> Self {
        Self { state_info }
    }
}

impl Widget for SettingsView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let content = match self.state_info {
            Some(info) => {
                let lines = vec![
                    Line::from(vec![Span::styled(
                        "Migrations",
                        Style::default().bold().fg(Color::Yellow),
                    )]),
                    Line::from(format!(
                        "  Current:  {}/{}",
                        info.migration_current, info.migration_total
                    )),
                    Line::from(""),
                    Line::from(vec![Span::styled(
                        "Storage",
                        Style::default().bold().fg(Color::Yellow),
                    )]),
                    Line::from(format!("  Executable:     {}", info.executable.display())),
                    Line::from(format!("  Data storage:   {}", info.data_dir.display())),
                    Line::from(format!("  Image layers:   {}", info.layers_dir.display())),
                    Line::from(format!(
                        "  Image metadata: {}",
                        info.metadata_file.display()
                    )),
                ];
                Text::from(lines)
            }
            None => Text::from("Loading state information..."),
        };

        Paragraph::new(content).render(area, buf);
    }
}
