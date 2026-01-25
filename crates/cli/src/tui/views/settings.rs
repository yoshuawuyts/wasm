use ratatui::{
    prelude::*,
    widgets::{Paragraph, Widget},
};

pub(crate) struct SettingsView;

impl Widget for SettingsView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Settings will be shown here...")
            .centered()
            .render(area, buf);
    }
}
