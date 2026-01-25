use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    prelude::*,
    widgets::{Block, Paragraph, Widget},
};
use std::time::Duration;
use tokio::sync::mpsc;

use super::{AppEvent, ManagerEvent};

const LOGO: &str = "
▖  ▖       
▌▞▖▌▀▌▛▘▛▛▌
▛ ▝▌█▌▄▌▌▌▌
";

pub(crate) struct App {
    running: bool,
    manager_ready: bool,
    app_sender: mpsc::Sender<AppEvent>,
    manager_receiver: mpsc::Receiver<ManagerEvent>,
}

impl App {
    pub(crate) fn new(
        app_sender: mpsc::Sender<AppEvent>,
        manager_receiver: mpsc::Receiver<ManagerEvent>,
    ) -> Self {
        Self {
            running: true,
            manager_ready: false,
            app_sender,
            manager_receiver,
        }
    }

    pub(crate) fn run(mut self, mut terminal: ratatui::DefaultTerminal) -> std::io::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
            self.handle_manager_events();
        }
        // Notify manager that we're quitting
        let _ = self.app_sender.try_send(AppEvent::Quit);
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        // Poll with a timeout so we can also check manager events
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key(key_event.code, key_event.modifiers);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_manager_events(&mut self) {
        while let Ok(event) = self.manager_receiver.try_recv() {
            match event {
                ManagerEvent::Ready => {
                    self.manager_ready = true;
                }
            }
        }
    }

    fn handle_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        match (key, modifiers) {
            (KeyCode::Char('q'), _) | (KeyCode::Esc, _) => self.running = false,
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.running = false,
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let status = if self.manager_ready {
            "ready"
        } else {
            "loading..."
        };
        let block = Block::bordered().title(format!(" wasm - {} ", status));
        Paragraph::new(LOGO.trim())
            .centered()
            .block(block)
            .render(area, buf);
    }
}
