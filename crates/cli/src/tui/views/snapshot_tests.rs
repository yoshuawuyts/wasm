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
use oci_client::manifest::{OciDescriptor, OciImageManifest};
use ratatui::prelude::*;
use std::path::PathBuf;
use wasm_package_manager::{ImageEntry, KnownPackage, StateInfo};

use super::{
    InterfacesView, LocalView, PackageDetailView, PackagesView, SearchView, SearchViewState,
    SettingsView,
};
use crate::tui::components::TabBar;
use crate::tui::views::packages::PackagesViewState;

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

/// Create a test ImageEntry for snapshot testing.
fn create_test_image_entry(
    registry: &str,
    repository: &str,
    tag: Option<&str>,
    digest: Option<&str>,
    size: u64,
) -> ImageEntry {
    ImageEntry::new_for_test(
        1,
        registry.to_string(),
        repository.to_string(),
        None,
        tag.map(|t| t.to_string()),
        digest.map(|d| d.to_string()),
        OciImageManifest {
            schema_version: 2,
            media_type: Some("application/vnd.oci.image.manifest.v1+json".to_string()),
            config: OciDescriptor {
                media_type: "application/vnd.oci.image.config.v1+json".to_string(),
                digest: "sha256:abc123".to_string(),
                size: 1024,
                urls: None,
                annotations: None,
            },
            layers: vec![OciDescriptor {
                media_type: "application/vnd.oci.image.layer.v1.tar+gzip".to_string(),
                digest: "sha256:layer123".to_string(),
                size: size as i64,
                urls: None,
                annotations: None,
            }],
            annotations: None,
            subject: None,
            artifact_type: None,
        },
        size,
    )
}

/// Create a test KnownPackage for snapshot testing.
fn create_test_known_package(
    registry: &str,
    repository: &str,
    tags: Vec<&str>,
    last_seen: &str,
) -> KnownPackage {
    KnownPackage::new_for_test(
        1,
        registry.to_string(),
        repository.to_string(),
        None,
        tags.into_iter().map(|t| t.to_string()).collect(),
        Vec::new(),
        Vec::new(),
        last_seen.to_string(),
        last_seen.to_string(),
    )
}

/// Create test StateInfo for snapshot testing.
fn create_test_state_info() -> StateInfo {
    StateInfo::new_for_test(
        PathBuf::from("/usr/local/bin/wasm"),
        PathBuf::from("/home/user/.local/share/wasm"),
        PathBuf::from("/home/user/.local/share/wasm/layers"),
        1024 * 1024 * 50, // 50 MB
        PathBuf::from("/home/user/.local/share/wasm/metadata.db"),
        1024 * 100, // 100 KB
        3,
        3,
    )
}

// =============================================================================
// LocalView Snapshot Tests
// =============================================================================

#[test]
fn test_local_view_snapshot() {
    let output = render_to_string(LocalView, 40, 10);
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
    let packages: Vec<ImageEntry> = vec![];
    let output = render_to_string(PackagesView::new(&packages), 80, 15);
    assert_snapshot!(output);
}

#[test]
fn test_packages_view_with_packages_snapshot() {
    let packages = vec![
        create_test_image_entry(
            "ghcr.io",
            "example/hello-world",
            Some("v1.0.0"),
            Some("sha256:abc123def456"),
            1024 * 1024 * 10, // 10 MB
        ),
        create_test_image_entry(
            "docker.io",
            "library/nginx",
            Some("latest"),
            Some("sha256:xyz789"),
            1024 * 1024 * 50, // 50 MB
        ),
    ];
    let output = render_to_string(PackagesView::new(&packages), 100, 15);
    assert_snapshot!(output);
}

#[test]
fn test_packages_view_with_filter_active_snapshot() {
    let packages = vec![create_test_image_entry(
        "ghcr.io",
        "example/hello-world",
        Some("v1.0.0"),
        Some("sha256:abc123def456"),
        1024 * 1024 * 10,
    )];
    let mut state = PackagesViewState::new();
    state.filter_active = true;
    state.filter_query = "hello".to_string();
    let output = render_stateful_to_string(PackagesView::new(&packages), &mut state, 100, 15);
    assert_snapshot!(output);
}

// =============================================================================
// PackageDetailView Snapshot Tests
// =============================================================================

#[test]
fn test_package_detail_view_snapshot() {
    let package = create_test_image_entry(
        "ghcr.io",
        "example/hello-world",
        Some("v1.0.0"),
        Some("sha256:abc123def456"),
        1024 * 1024 * 10,
    );
    let output = render_to_string(PackageDetailView::new(&package), 80, 25);
    assert_snapshot!(output);
}

#[test]
fn test_package_detail_view_minimal_snapshot() {
    let package = create_test_image_entry("ghcr.io", "minimal/package", None, None, 1024);
    let output = render_to_string(PackageDetailView::new(&package), 80, 25);
    assert_snapshot!(output);
}

// =============================================================================
// SearchView Snapshot Tests
// =============================================================================

#[test]
fn test_search_view_empty_snapshot() {
    let packages: Vec<KnownPackage> = vec![];
    let output = render_to_string(SearchView::new(&packages), 80, 15);
    assert_snapshot!(output);
}

#[test]
fn test_search_view_with_packages_snapshot() {
    let packages = vec![
        create_test_known_package(
            "ghcr.io",
            "example/hello-world",
            vec!["v1.0.0", "v1.1.0", "latest"],
            "2024-01-15T10:30:00Z",
        ),
        create_test_known_package(
            "docker.io",
            "library/nginx",
            vec!["1.24", "1.25", "stable", "latest", "mainline"],
            "2024-01-14T08:00:00Z",
        ),
    ];
    let output = render_to_string(SearchView::new(&packages), 100, 15);
    assert_snapshot!(output);
}

#[test]
fn test_search_view_with_search_active_snapshot() {
    let packages = vec![create_test_known_package(
        "ghcr.io",
        "example/hello-world",
        vec!["v1.0.0"],
        "2024-01-15T10:30:00Z",
    )];
    let mut state = SearchViewState::new();
    state.search_active = true;
    state.search_query = "hello".to_string();
    let output = render_stateful_to_string(SearchView::new(&packages), &mut state, 100, 15);
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

#[test]
fn test_settings_view_with_state_info_snapshot() {
    let state_info = create_test_state_info();
    let output = render_to_string(SettingsView::new(Some(&state_info)), 100, 15);
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

impl crate::tui::components::TabItem for TestTab {
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
