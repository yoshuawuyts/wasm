use ratatui::{
    prelude::*,
    widgets::{Block, List, ListItem, Paragraph, Widget},
};
use wasm_detector::{InterfaceInfo, InterfaceKind, WasmEntry};

/// View for the Interfaces tab
#[derive(Debug)]
pub struct InterfacesView<'a> {
    wasm_files: &'a [WasmEntry],
}

impl<'a> InterfacesView<'a> {
    /// Create a new InterfacesView with the given WASM files
    #[must_use]
    pub fn new(wasm_files: &'a [WasmEntry]) -> Self {
        Self { wasm_files }
    }
}

impl Widget for InterfacesView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Collect all unique interfaces from all WASM files
        let mut all_interfaces: Vec<(&InterfaceInfo, &WasmEntry)> = Vec::new();

        for wasm_entry in self.wasm_files {
            for interface in wasm_entry.interfaces() {
                all_interfaces.push((interface, wasm_entry));
            }
        }

        if all_interfaces.is_empty() {
            let message = Paragraph::new(
                "No WIT interfaces detected in local WASM files.\n\nPress 'r' to refresh.",
            )
            .centered()
            .style(Style::default().fg(Color::DarkGray));
            message.render(area, buf);
        } else {
            #[allow(clippy::indexing_slicing)]
            let layout = Layout::vertical([
                Constraint::Length(2),
                Constraint::Min(0),
                Constraint::Length(2),
            ])
            .split(area);

            // Title
            let title = Paragraph::new(format!(
                "Detected {} interface(s) from {} WASM file(s)",
                all_interfaces.len(),
                self.wasm_files.len()
            ))
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center);
            #[allow(clippy::indexing_slicing)]
            title.render(layout[0], buf);

            // List of interfaces
            let items: Vec<ListItem> = all_interfaces
                .iter()
                .enumerate()
                .map(|(idx, (interface, wasm_entry))| {
                    let kind_str = match interface.kind {
                        InterfaceKind::Dependency => "dep",
                        InterfaceKind::ChildComponent => "component",
                        InterfaceKind::ChildModule => "module",
                    };

                    let version_str = interface
                        .version
                        .as_ref()
                        .map(|v| format!(" [{}]", v))
                        .unwrap_or_default();

                    let file_name = wasm_entry.file_name().unwrap_or("<unnamed>");
                    let content = format!(
                        "{}. {} ({}) from {}{}",
                        idx + 1,
                        interface.name,
                        kind_str,
                        file_name,
                        version_str
                    );
                    ListItem::new(content).style(Style::default().fg(Color::White))
                })
                .collect();

            let list = List::new(items)
                .block(Block::bordered().title("Interfaces"))
                .style(Style::default());

            #[allow(clippy::indexing_slicing)]
            Widget::render(list, layout[1], buf);

            // Help text
            let help = Paragraph::new("Press 'r' to refresh • 'q' to quit")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);
            #[allow(clippy::indexing_slicing)]
            help.render(layout[2], buf);
        }
    }
}
