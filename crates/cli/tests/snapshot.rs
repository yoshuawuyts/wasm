//! Snapshot tests for TUI views using the `insta` crate.
//!
//! These tests render each view to a buffer and snapshot the result to ensure
//! consistent rendering across changes.
//!
//! # Running Snapshot Tests
//!
//! Run tests with: `cargo test --package wasm`
//!
//! # Updating Snapshots
//!
//! When views change intentionally, update snapshots with:
//! `cargo insta review` or `cargo insta accept`
//!
//! Install the insta CLI with: `cargo install cargo-insta`

use insta::assert_snapshot;
use ratatui::prelude::*;

use wasm::tui::components::{TabBar, TabItem};
use wasm::tui::views::packages::PackagesViewState;
use wasm::tui::views::{
    InterfacesView, KnownPackageDetailView, LocalView, PackagesView, SearchView, SearchViewState,
    SettingsView,
};
use wasm_package_manager::KnownPackage;

/// Helper function to render a widget to a string buffer.
fn render_to_string<W: Widget>(widget: W, width: u16, height: u16) -> String {
    let area = Rect::new(0, 0, width, height);
    let mut buffer = Buffer::empty(area);
    widget.render(area, &mut buffer);
    buffer_to_string(&buffer)
}

/// Helper function to render a stateful widget to a string buffer.
fn render_stateful_to_string<W, S>(widget: W, state: &mut S, width: u16, height: u16) -> String
where
    W: StatefulWidget<State = S>,
{
    let area = Rect::new(0, 0, width, height);
    let mut buffer = Buffer::empty(area);
    widget.render(area, &mut buffer, state);
    buffer_to_string(&buffer)
}

/// Convert a buffer to a string representation for snapshot testing.
fn buffer_to_string(buffer: &Buffer) -> String {
    let mut output = String::new();
    for y in 0..buffer.area.height {
        let line_start = output.len();
        for x in 0..buffer.area.width {
            let cell = &buffer[(x, y)];
            output.push_str(cell.symbol());
        }
        // Trim trailing spaces using truncate to avoid allocation
        let trimmed_len = output[line_start..].trim_end().len() + line_start;
        output.truncate(trimmed_len);
        output.push('\n');
    }
    output
}

// =============================================================================
// LocalView Snapshot Tests
// =============================================================================

#[test]
fn test_local_view_snapshot() {
    let wasm_files = vec![];
    let output = render_to_string(LocalView::new(&wasm_files), 40, 10);
    assert_snapshot!(output);
}

// =============================================================================
// InterfacesView Snapshot Tests
// =============================================================================

#[test]
fn test_interfaces_view_snapshot() {
    let output = render_to_string(InterfacesView, 60, 10);
    assert_snapshot!(output);
}

// =============================================================================
// PackagesView Snapshot Tests
// =============================================================================

#[test]
fn test_packages_view_empty_snapshot() {
    let packages = vec![];
    let output = render_to_string(PackagesView::new(&packages), 80, 15);
    assert_snapshot!(output);
}

#[test]
fn test_packages_view_with_filter_active_snapshot() {
    let packages = vec![];
    let mut state = PackagesViewState::new();
    state.filter_active = true;
    state.filter_query = "wasi".to_string();
    let output = render_stateful_to_string(PackagesView::new(&packages), &mut state, 100, 15);
    assert_snapshot!(output);
}

// =============================================================================
// SearchView Snapshot Tests
// =============================================================================

#[test]
fn test_search_view_empty_snapshot() {
    let packages = vec![];
    let output = render_to_string(SearchView::new(&packages), 80, 15);
    assert_snapshot!(output);
}

#[test]
fn test_search_view_with_search_active_snapshot() {
    let packages = vec![];
    let mut state = SearchViewState::new();
    state.search_active = true;
    state.search_query = "wasi".to_string();
    let output = render_stateful_to_string(SearchView::new(&packages), &mut state, 100, 15);
    assert_snapshot!(output);
}

// =============================================================================
// KnownPackageDetailView Snapshot Tests
// =============================================================================

#[test]
fn test_known_package_detail_view_snapshot() {
    let package = KnownPackage::new_for_testing(
        "ghcr.io".to_string(),
        "user/example-package".to_string(),
        Some("An example WASM component package".to_string()),
        vec![
            "v1.0.0".to_string(),
            "v0.9.0".to_string(),
            "latest".to_string(),
        ],
        vec!["v1.0.0.sig".to_string()],
        vec!["v1.0.0.att".to_string()],
        "2024-01-15T10:30:00Z".to_string(),
        "2024-01-01T08:00:00Z".to_string(),
    );
    let output = render_to_string(KnownPackageDetailView::new(&package), 80, 20);
    assert_snapshot!(output);
}

// =============================================================================
// SettingsView Snapshot Tests
// =============================================================================

#[test]
fn test_settings_view_loading_snapshot() {
    let output = render_to_string(SettingsView::new(None), 80, 15);
    assert_snapshot!(output);
}

// =============================================================================
// TabBar Component Snapshot Tests
// =============================================================================

/// Tab enum for testing the TabBar component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TestTab {
    First,
    Second,
    Third,
}

impl TestTab {
    const ALL: [TestTab; 3] = [TestTab::First, TestTab::Second, TestTab::Third];
}

impl TabItem for TestTab {
    fn all() -> &'static [Self] {
        &Self::ALL
    }

    fn title(&self) -> &'static str {
        match self {
            TestTab::First => "First [1]",
            TestTab::Second => "Second [2]",
            TestTab::Third => "Third [3]",
        }
    }
}

#[test]
fn test_tab_bar_first_selected_snapshot() {
    let tab_bar = TabBar::new("Test App - ready", TestTab::First);
    let output = render_to_string(tab_bar, 60, 3);
    assert_snapshot!(output);
}

#[test]
fn test_tab_bar_second_selected_snapshot() {
    let tab_bar = TabBar::new("Test App - ready", TestTab::Second);
    let output = render_to_string(tab_bar, 60, 3);
    assert_snapshot!(output);
}

#[test]
fn test_tab_bar_loading_state_snapshot() {
    let tab_bar = TabBar::new("Test App - loading...", TestTab::First);
    let output = render_to_string(tab_bar, 60, 3);
    assert_snapshot!(output);
}
