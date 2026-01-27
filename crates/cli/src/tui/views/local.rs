use ratatui::{
    prelude::*,
    widgets::{Block, List, ListItem, Paragraph, StatefulWidget, Widget},
};
use std::path::PathBuf;

/// Represents a detected local wasm file
#[derive(Debug, Clone)]
pub struct LocalWasmFile {
    /// The filesystem path to the wasm file
    pub path: PathBuf,
}

impl LocalWasmFile {
    /// Get the file name
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.path.file_name().and_then(|s| s.to_str())
    }
}

/// State for the local view
#[derive(Debug, Clone, Default)]
pub struct LocalViewState {
    /// List of detected wasm files
    pub files: Vec<LocalWasmFile>,
    /// Currently selected file index
    pub selected: usize,
    /// Whether the view is loading
    pub loading: bool,
}

impl LocalViewState {
    /// Create a new local view state
    #[must_use]
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            selected: 0,
            loading: true,
        }
    }

    /// Select the next file in the list
    pub fn select_next(&mut self) {
        if !self.files.is_empty() {
            self.selected = (self.selected + 1) % self.files.len();
        }
    }

    /// Select the previous file in the list
    pub fn select_prev(&mut self) {
        if !self.files.is_empty() {
            self.selected = if self.selected == 0 {
                self.files.len() - 1
            } else {
                self.selected - 1
            };
        }
    }
}

/// Widget for displaying local wasm files
pub struct LocalView<'a> {
    files: &'a [LocalWasmFile],
}

impl<'a> std::fmt::Debug for LocalView<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalView")
            .field("files_count", &self.files.len())
            .finish()
    }
}

impl<'a> LocalView<'a> {
    /// Create a new local view with the given files
    #[must_use]
    pub fn new(files: &'a [LocalWasmFile]) -> Self {
        Self { files }
    }
}

impl StatefulWidget for LocalView<'_> {
    type State = LocalViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::bordered().title(" Local WASM Files ");

        if state.loading {
            let loading_msg = Paragraph::new("Loading local WASM files...")
                .centered()
                .block(block);
            loading_msg.render(area, buf);
            return;
        }

        if self.files.is_empty() {
            let empty_msg =
                Paragraph::new("No .wasm files found in current directory\n\nPress 'r' to refresh")
                    .centered()
                    .block(block);
            empty_msg.render(area, buf);
            return;
        }

        let inner = block.inner(area);
        block.render(area, buf);

        // Create list items
        let items: Vec<ListItem> = self
            .files
            .iter()
            .enumerate()
            .map(|(i, file)| {
                let file_name = file.file_name().unwrap_or("Unknown");
                let path_str = file.path.display().to_string();

                let content = if i == state.selected {
                    format!("▶ {} ({})", file_name, path_str)
                } else {
                    format!("  {} ({})", file_name, path_str)
                };

                let style = if i == state.selected {
                    Style::default().fg(Color::Yellow).bold()
                } else {
                    Style::default()
                };

                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(items);
        Widget::render(list, inner, buf);

        // Instructions at the bottom
        let instructions = format!(
            "↑/↓: Navigate | r: Refresh | Found {} file(s)",
            self.files.len()
        );
        let instructions_area = Rect {
            x: inner.x,
            y: inner.y + inner.height.saturating_sub(1),
            width: inner.width,
            height: 1,
        };
        Paragraph::new(instructions)
            .style(Style::default().fg(Color::DarkGray))
            .render(instructions_area, buf);
    }
}
