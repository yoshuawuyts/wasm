use ratatui::{
    prelude::*,
    widgets::{Paragraph, Widget},
};

pub struct InterfacesView;

impl Widget for InterfacesView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Interfaces will be shown here...")
            .centered()
            .render(area, buf);
    }
}
