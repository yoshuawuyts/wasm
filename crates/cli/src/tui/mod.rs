use ratatui::{crossterm, widgets::Paragraph};

const LOGO: &str = "
▖  ▖       
▌▞▖▌▀▌▛▘▛▛▌
▛ ▝▌█▌▄▌▌▌▌
";

pub async fn run() -> anyhow::Result<()> {
    ratatui::run(|terminal| {
        loop {
            terminal
                .draw(|frame| frame.render_widget(Paragraph::new(LOGO.trim()), frame.area()))?;
            if crossterm::event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}
