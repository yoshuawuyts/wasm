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

/// View for the local tab.
#[derive(Debug)]
pub struct LocalView;

impl Widget for LocalView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(LOGO.trim()).centered().render(area, buf);
    }
}
