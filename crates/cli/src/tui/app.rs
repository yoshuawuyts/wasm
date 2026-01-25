use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    prelude::*,
    widgets::{Block, List, ListItem, Paragraph, Tabs, Widget},
};
use std::time::Duration;
use tokio::sync::mpsc;
use wasm_package_manager::ImageEntry;

use super::{AppEvent, ManagerEvent};

const LOGO: &str = "
▖  ▖       
▌▞▖▌▀▌▛▘▛▛▌
▛ ▝▌█▌▄▌▌▌▌
";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tab {
    Home,
    Packages,
    Settings,
}

impl Tab {
    const ALL: [Tab; 3] = [Tab::Home, Tab::Packages, Tab::Settings];

    fn title(self) -> &'static str {
        match self {
            Tab::Home => "Home",
            Tab::Packages => "Packages",
            Tab::Settings => "Settings",
        }
    }

    fn next(self) -> Self {
        match self {
            Tab::Home => Tab::Packages,
            Tab::Packages => Tab::Settings,
            Tab::Settings => Tab::Home,
        }
    }

    fn prev(self) -> Self {
        match self {
            Tab::Home => Tab::Settings,
            Tab::Packages => Tab::Home,
            Tab::Settings => Tab::Packages,
        }
    }
}

pub(crate) struct App {
    running: bool,
    manager_ready: bool,
    current_tab: Tab,
    packages: Vec<ImageEntry>,
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
            current_tab: Tab::Home,
            packages: Vec::new(),
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
                    // Request packages list when manager is ready
                    let _ = self.app_sender.try_send(AppEvent::RequestPackages);
                }
                ManagerEvent::PackagesList(packages) => {
                    self.packages = packages;
                }
            }
        }
    }

    fn handle_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        match (key, modifiers) {
            (KeyCode::Char('q'), _) | (KeyCode::Esc, _) => self.running = false,
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.running = false,
            // Tab navigation
            (KeyCode::Tab, KeyModifiers::NONE) | (KeyCode::Right, _) => {
                self.current_tab = self.current_tab.next();
            }
            (KeyCode::BackTab, _) | (KeyCode::Left, _) => {
                self.current_tab = self.current_tab.prev();
            }
            (KeyCode::Char('1'), _) => self.current_tab = Tab::Home,
            (KeyCode::Char('2'), _) => self.current_tab = Tab::Packages,
            (KeyCode::Char('3'), _) => self.current_tab = Tab::Settings,
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

        // Create main layout with tabs at top
        let layout = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(area);

        // Render tab bar
        let tab_titles: Vec<&str> = Tab::ALL.iter().map(|t| t.title()).collect();
        let selected_index = Tab::ALL
            .iter()
            .position(|&t| t == self.current_tab)
            .unwrap_or(0);
        let tabs = Tabs::new(tab_titles)
            .block(Block::bordered().title(format!(" wasm - {} ", status)))
            .highlight_style(Style::default().bold().fg(Color::Yellow))
            .select(selected_index);
        tabs.render(layout[0], buf);

        // Render content based on current tab
        let content_block = Block::bordered();
        let content_area = content_block.inner(layout[1]);
        content_block.render(layout[1], buf);

        match self.current_tab {
            Tab::Home => {
                Paragraph::new(LOGO.trim())
                    .centered()
                    .render(content_area, buf);
            }
            Tab::Packages => {
                if self.packages.is_empty() {
                    Paragraph::new("No packages stored.")
                        .centered()
                        .render(content_area, buf);
                } else {
                    let items: Vec<ListItem> = self
                        .packages
                        .iter()
                        .map(|entry| {
                            let tag = entry.ref_tag.as_deref().unwrap_or("<none>");
                            let digest = entry
                                .ref_digest
                                .as_ref()
                                .map(|d| {
                                    if d.len() > 16 {
                                        format!("{}...", &d[..16])
                                    } else {
                                        d.clone()
                                    }
                                })
                                .unwrap_or_else(|| "<none>".to_string());
                            let text = format!(
                                "{}/{}:{} ({})",
                                entry.ref_registry, entry.ref_repository, tag, digest
                            );
                            ListItem::new(text)
                        })
                        .collect();
                    let list = List::new(items);
                    Widget::render(list, content_area, buf);
                }
            }
            Tab::Settings => {
                Paragraph::new("Settings will be shown here...")
                    .centered()
                    .render(content_area, buf);
            }
        }
    }
}
