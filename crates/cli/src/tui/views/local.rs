use ratatui::{
    prelude::*,
    widgets::{Paragraph, Widget},
};

const LOGO: &str = "
▖  ▖       ▐▘▗ ▜ 
▌▞▖▌▀▌▛▘▛▛▌▐ ▜ ▐ 
▛ ▝▌█▌▄▌▌▌▌▐ ▟▖▐ 
           ▝▘  ▀ 
";

/// View for the Local tab showing detected WASM files
#[derive(Debug)]
pub struct LocalView;

impl Widget for LocalView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(LOGO.trim()).centered().render(area, buf);
    }
}
