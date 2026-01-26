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

pub struct LocalView;

impl Widget for LocalView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(LOGO.trim()).centered().render(area, buf);
    }
}
