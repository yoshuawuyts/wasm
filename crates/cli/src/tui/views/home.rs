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

pub(crate) struct HomeView;

impl Widget for HomeView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(LOGO.trim()).centered().render(area, buf);
    }
}
