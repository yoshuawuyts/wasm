use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    prelude::*,
    widgets::{Block, Clear, Paragraph},
};
use std::time::Duration;
use tokio::sync::mpsc;
use wasm_package_manager::{ImageEntry, InsertResult, KnownPackage, StateInfo};

use super::components::{TabBar, TabItem};
use super::views::packages::PackagesViewState;
use super::views::{
    InterfacesView, LocalView, PackageDetailView, PackagesView, SearchView, SearchViewState,
    SettingsView,
};
use super::{AppEvent, ManagerEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tab {
    Local,
    Components,
    Interfaces,
    Search,
    Settings,
}

impl Tab {
    const ALL: [Tab; 5] = [
        Tab::Local,
        Tab::Components,
        Tab::Interfaces,
        Tab::Search,
        Tab::Settings,
    ];
}

impl TabItem for Tab {
    fn all() -> &'static [Self] {
        &Self::ALL
    }

    fn title(&self) -> &'static str {
        match self {
            Tab::Local => "Local",
            Tab::Components => "Components",
            Tab::Interfaces => "Interfaces",
            Tab::Search => "Search",
            Tab::Settings => "Settings",
        }
    }
}

pub(crate) struct App {
    running: bool,
    manager_ready: bool,
    current_tab: Tab,
    packages: Vec<ImageEntry>,
    packages_view_state: PackagesViewState,
    /// When Some, we're viewing a package detail
    viewing_package: Option<usize>,
    /// State info from the manager
    state_info: Option<StateInfo>,
    /// Pull prompt state
    pull_prompt_active: bool,
    pull_prompt_input: String,
    pull_prompt_error: Option<String>,
    pull_in_progress: bool,
    /// Search view state
    search_view_state: SearchViewState,
    /// Known packages for search results
    known_packages: Vec<KnownPackage>,
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
            current_tab: Tab::Local,
            packages: Vec::new(),
            packages_view_state: PackagesViewState::new(),
            viewing_package: None,
            state_info: None,
            pull_prompt_active: false,
            pull_prompt_input: String::new(),
            pull_prompt_error: None,
            pull_in_progress: false,
            search_view_state: SearchViewState::new(),
            known_packages: Vec::new(),
            app_sender,
            manager_receiver,
        }
    }

    pub(crate) fn run(mut self, mut terminal: ratatui::DefaultTerminal) -> std::io::Result<()> {
        while self.running {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
            self.handle_manager_events();
        }
        // Notify manager that we're quitting
        let _ = self.app_sender.try_send(AppEvent::Quit);
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut ratatui::Frame) {
        let area = frame.area();
        let status = if self.manager_ready {
            "ready"
        } else {
            "loading..."
        };

        // Create main layout with tabs at top
        let layout = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(area);

        // Render tab bar
        let tab_bar = TabBar::new(format!("wasm - {}", status), self.current_tab);
        frame.render_widget(tab_bar, layout[0]);

        // Render content based on current tab
        let content_block = Block::bordered();
        let content_area = content_block.inner(layout[1]);
        frame.render_widget(content_block, layout[1]);

        match self.current_tab {
            Tab::Local => frame.render_widget(LocalView, content_area),
            Tab::Components => {
                // Check if we're viewing a package detail
                if let Some(idx) = self.viewing_package {
                    if let Some(package) = self.packages.get(idx) {
                        frame.render_widget(PackageDetailView::new(package), content_area);
                    }
                } else {
                    frame.render_stateful_widget(
                        PackagesView::new(&self.packages),
                        content_area,
                        &mut self.packages_view_state,
                    );
                }
            }
            Tab::Interfaces => frame.render_widget(InterfacesView, content_area),
            Tab::Search => {
                frame.render_stateful_widget(
                    SearchView::new(&self.known_packages),
                    content_area,
                    &mut self.search_view_state,
                );
            }
            Tab::Settings => {
                frame.render_widget(SettingsView::new(self.state_info.as_ref()), content_area)
            }
        }

        // Render pull prompt overlay if active
        if self.pull_prompt_active {
            self.render_pull_prompt(frame, area);
        }
    }

    fn render_pull_prompt(&self, frame: &mut ratatui::Frame, area: Rect) {
        // Calculate centered popup area
        let popup_width = 60.min(area.width.saturating_sub(4));
        let popup_height = if self.pull_prompt_error.is_some() {
            7
        } else {
            5
        };
        let popup_area = Rect {
            x: (area.width.saturating_sub(popup_width)) / 2,
            y: (area.height.saturating_sub(popup_height)) / 2,
            width: popup_width,
            height: popup_height,
        };

        // Clear the area behind the popup
        frame.render_widget(Clear, popup_area);

        // Build the prompt content
        let title = if self.pull_in_progress {
            " Pull Package (pulling...) "
        } else {
            " Pull Package "
        };

        let block = Block::bordered()
            .title(title)
            .style(Style::default().bg(Color::DarkGray));

        let inner = block.inner(popup_area);
        frame.render_widget(block, popup_area);

        // Layout for input and optional error
        let chunks = if self.pull_prompt_error.is_some() {
            Layout::vertical([
                Constraint::Length(1), // Label
                Constraint::Length(1), // Input
                Constraint::Length(1), // Error
            ])
            .split(inner)
        } else {
            Layout::vertical([
                Constraint::Length(1), // Label
                Constraint::Length(1), // Input
            ])
            .split(inner)
        };

        // Label
        let label = Paragraph::new("Enter package reference (e.g., ghcr.io/user/pkg:tag):");
        frame.render_widget(label, chunks[0]);

        // Input field with cursor
        let input_text = format!("{}_", self.pull_prompt_input);
        let input = Paragraph::new(input_text).style(Style::default().fg(Color::Yellow));
        frame.render_widget(input, chunks[1]);

        // Error message if present
        if let Some(ref error) = self.pull_prompt_error {
            let error_msg = Paragraph::new(error.as_str()).style(Style::default().fg(Color::Red));
            frame.render_widget(error_msg, chunks[2]);
        }
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
                    // Request packages list and state info when manager is ready
                    let _ = self.app_sender.try_send(AppEvent::RequestPackages);
                    let _ = self.app_sender.try_send(AppEvent::RequestStateInfo);
                    let _ = self.app_sender.try_send(AppEvent::RequestKnownPackages);
                }
                ManagerEvent::PackagesList(packages) => {
                    self.packages = packages;
                }
                ManagerEvent::StateInfo(state_info) => {
                    self.state_info = Some(state_info);
                }
                ManagerEvent::PullResult(result) => {
                    self.pull_in_progress = false;
                    match result {
                        Ok(insert_result) => {
                            // Close the prompt on success
                            self.pull_prompt_active = false;
                            self.pull_prompt_input.clear();
                            if insert_result == InsertResult::AlreadyExists {
                                // Show a warning that the package already exists
                                self.pull_prompt_error = Some(
                                    "Warning: package already exists in local store".to_string(),
                                );
                            } else {
                                self.pull_prompt_error = None;
                            }
                            // Refresh known packages
                            let _ = self.app_sender.try_send(AppEvent::RequestKnownPackages);
                        }
                        Err(e) => {
                            self.pull_prompt_error = Some(e);
                        }
                    }
                }
                ManagerEvent::DeleteResult(_result) => {
                    // Delete completed, packages list will be refreshed automatically
                }
                ManagerEvent::SearchResults(packages) => {
                    self.known_packages = packages;
                }
                ManagerEvent::KnownPackagesList(packages) => {
                    self.known_packages = packages;
                }
            }
        }
    }

    fn handle_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        // Handle pull prompt input first
        if self.pull_prompt_active {
            self.handle_pull_prompt_key(key, modifiers);
            return;
        }

        // Handle search input mode
        if self.current_tab == Tab::Search && self.search_view_state.search_active {
            self.handle_search_key(key, modifiers);
            return;
        }

        // If viewing a package detail, handle back navigation
        if self.viewing_package.is_some() {
            match key {
                KeyCode::Esc | KeyCode::Backspace => {
                    self.viewing_package = None;
                }
                KeyCode::Char('q') => self.running = false,
                KeyCode::Char('c') if modifiers == KeyModifiers::CONTROL => self.running = false,
                _ => {}
            }
            return;
        }

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
            (KeyCode::Char('1'), _) => self.current_tab = Tab::Local,
            (KeyCode::Char('2'), _) => self.current_tab = Tab::Components,
            (KeyCode::Char('3'), _) => self.current_tab = Tab::Interfaces,
            (KeyCode::Char('4'), _) => self.current_tab = Tab::Search,
            (KeyCode::Char('5'), _) => self.current_tab = Tab::Settings,
            // Pull prompt - 'p' to open
            (KeyCode::Char('p'), _) if self.manager_ready => {
                self.pull_prompt_active = true;
                self.pull_prompt_input.clear();
                self.pull_prompt_error = None;
            }
            // Package list navigation (when on Components tab)
            (KeyCode::Up, _) | (KeyCode::Char('k'), _) if self.current_tab == Tab::Components => {
                self.packages_view_state.select_prev(self.packages.len());
            }
            (KeyCode::Down, _) | (KeyCode::Char('j'), _) if self.current_tab == Tab::Components => {
                self.packages_view_state.select_next(self.packages.len());
            }
            (KeyCode::Enter, _) if self.current_tab == Tab::Components => {
                if let Some(selected) = self.packages_view_state.selected() {
                    if selected < self.packages.len() {
                        self.viewing_package = Some(selected);
                    }
                }
            }
            // Delete selected package
            (KeyCode::Char('d'), _)
                if self.current_tab == Tab::Components && self.manager_ready =>
            {
                if let Some(selected) = self.packages_view_state.selected() {
                    if let Some(package) = self.packages.get(selected) {
                        let _ = self
                            .app_sender
                            .try_send(AppEvent::Delete(package.reference()));
                        // Adjust selection if we're deleting the last item
                        if selected > 0 && selected >= self.packages.len() - 1 {
                            self.packages_view_state
                                .table_state
                                .select(Some(selected - 1));
                        }
                    }
                }
            }
            // Search tab navigation
            (KeyCode::Up, _) | (KeyCode::Char('k'), _) if self.current_tab == Tab::Search => {
                self.search_view_state
                    .select_prev(self.known_packages.len());
            }
            (KeyCode::Down, _) | (KeyCode::Char('j'), _) if self.current_tab == Tab::Search => {
                self.search_view_state
                    .select_next(self.known_packages.len());
            }
            // Activate search input with '/'
            (KeyCode::Char('/'), _) if self.current_tab == Tab::Search => {
                self.search_view_state.search_active = true;
            }
            // Pull selected package from search results
            (KeyCode::Enter, _) if self.current_tab == Tab::Search && self.manager_ready => {
                if let Some(selected) = self.search_view_state.selected() {
                    if let Some(package) = self.known_packages.get(selected) {
                        // Pull the package with the most recent tag (or latest if none)
                        let reference = package.reference_with_tag();
                        let _ = self.app_sender.try_send(AppEvent::Pull(reference));
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_search_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        match key {
            KeyCode::Esc => {
                // Exit search mode
                self.search_view_state.search_active = false;
            }
            KeyCode::Enter => {
                // Execute search
                self.search_view_state.search_active = false;
                if self.search_view_state.search_query.is_empty() {
                    let _ = self.app_sender.try_send(AppEvent::RequestKnownPackages);
                } else {
                    let _ = self.app_sender.try_send(AppEvent::SearchPackages(
                        self.search_view_state.search_query.clone(),
                    ));
                }
            }
            KeyCode::Backspace => {
                self.search_view_state.search_query.pop();
            }
            KeyCode::Char(c) => {
                if modifiers == KeyModifiers::CONTROL && c == 'c' {
                    self.running = false;
                } else {
                    self.search_view_state.search_query.push(c);
                }
            }
            _ => {}
        }
    }

    fn handle_pull_prompt_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        // Don't allow input while pull is in progress
        if self.pull_in_progress {
            return;
        }

        match key {
            KeyCode::Esc => {
                // Cancel the prompt
                self.pull_prompt_active = false;
                self.pull_prompt_input.clear();
                self.pull_prompt_error = None;
            }
            KeyCode::Enter => {
                if !self.pull_prompt_input.is_empty() {
                    // Send pull request to manager
                    self.pull_in_progress = true;
                    self.pull_prompt_error = None;
                    let _ = self
                        .app_sender
                        .try_send(AppEvent::Pull(self.pull_prompt_input.clone()));
                }
            }
            KeyCode::Backspace => {
                self.pull_prompt_input.pop();
                self.pull_prompt_error = None;
            }
            KeyCode::Char(c) => {
                if modifiers == KeyModifiers::CONTROL && c == 'c' {
                    self.running = false;
                } else {
                    self.pull_prompt_input.push(c);
                    self.pull_prompt_error = None;
                }
            }
            _ => {}
        }
    }
}
