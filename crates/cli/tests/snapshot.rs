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
//!
//! # Test Coverage Guidelines
//!
//! Every TUI view and component should have at least one snapshot test covering:
//! - Empty/loading state (when applicable)
//! - Populated state with sample data
//! - Interactive states (filter active, search active, etc.)
//!
//! When adding new views or components, add corresponding snapshot tests.

use std::path::PathBuf;

use insta::assert_snapshot;
use ratatui::prelude::*;

use wasm::tui::components::{TabBar, TabItem};
use wasm::tui::views::packages::PackagesViewState;
use wasm::tui::views::{
    InterfacesView, InterfacesViewState, KnownPackageDetailView, LocalView, PackageDetailView,
    PackagesView, SearchView, SearchViewState, SettingsView,
};
use wasm_detector::WasmEntry;
use wasm_package_manager::{ImageEntry, KnownPackage, StateInfo};

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
fn test_local_view_empty_snapshot() {
    let wasm_files = vec![];
    let output = render_to_string(LocalView::new(&wasm_files), 60, 10);
    assert_snapshot!(output);
}

#[test]
fn test_local_view_with_files_snapshot() {
    let wasm_files = vec![
        WasmEntry::new_for_testing(PathBuf::from(
            "./target/wasm32-unknown-unknown/release/app.wasm",
        )),
        WasmEntry::new_for_testing(PathBuf::from("./pkg/component.wasm")),
        WasmEntry::new_for_testing(PathBuf::from("./examples/hello.wasm")),
    ];
    let output = render_to_string(LocalView::new(&wasm_files), 80, 15);
    assert_snapshot!(output);
}

// =============================================================================
// InterfacesView Snapshot Tests
// =============================================================================

#[test]
fn test_interfaces_view_snapshot() {
    let interfaces = vec![];
    let mut state = InterfacesViewState::new();
    let output = render_stateful_to_string(InterfacesView::new(&interfaces), &mut state, 60, 10);
    assert_snapshot!(output);
}

#[test]
fn test_interfaces_view_populated_snapshot() {
    use semver::Version;
    use wasm_package_manager::WitInterface;
    use wit_parser::{
        Function, FunctionKind, Package, PackageName, Resolve, World, WorldItem, WorldKey,
    };

    // Create a proper WIT package structure using wit-parser
    let mut resolve = Resolve::default();

    // Create first package: wasi:http@0.2.0 with proxy world
    let http_package = Package {
        name: PackageName {
            namespace: "wasi".to_string(),
            name: "http".to_string(),
            version: Some(Version::new(0, 2, 0)),
        },
        docs: Default::default(),
        interfaces: Default::default(),
        worlds: Default::default(),
    };
    let http_package_id = resolve.packages.alloc(http_package);

    // Create proxy world
    let mut proxy_world = World {
        name: "proxy".to_string(),
        docs: Default::default(),
        imports: Default::default(),
        exports: Default::default(),
        includes: Default::default(),
        include_names: Default::default(),
        package: Some(http_package_id),
        stability: Default::default(),
    };

    // Add imports/exports
    proxy_world.imports.insert(
        WorldKey::Name("wasi:http/types".to_string()),
        WorldItem::Function(Function {
            name: "types".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![],
            result: None,
            docs: Default::default(),
            stability: Default::default(),
        }),
    );
    proxy_world.exports.insert(
        WorldKey::Name("wasi:http/handler".to_string()),
        WorldItem::Function(Function {
            name: "handler".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![],
            result: None,
            docs: Default::default(),
            stability: Default::default(),
        }),
    );

    let proxy_world_id = resolve.worlds.alloc(proxy_world);
    resolve.packages[http_package_id]
        .worlds
        .insert("proxy".to_string(), proxy_world_id);

    // Generate WIT text using wit-encoder
    let packages = wit_encoder::packages_from_parsed(&resolve);
    let http_wit_text = packages[0].to_string();

    // Create second package: wasi:cli@0.2.0 with command world
    let cli_package = Package {
        name: PackageName {
            namespace: "wasi".to_string(),
            name: "cli".to_string(),
            version: Some(Version::new(0, 2, 0)),
        },
        docs: Default::default(),
        interfaces: Default::default(),
        worlds: Default::default(),
    };
    let cli_package_id = resolve.packages.alloc(cli_package);

    // Create command world
    let mut command_world = World {
        name: "command".to_string(),
        docs: Default::default(),
        imports: Default::default(),
        exports: Default::default(),
        includes: Default::default(),
        include_names: Default::default(),
        package: Some(cli_package_id),
        stability: Default::default(),
    };

    // Add imports/exports
    command_world.imports.insert(
        WorldKey::Name("wasi:cli/stdin".to_string()),
        WorldItem::Function(Function {
            name: "stdin".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![],
            result: None,
            docs: Default::default(),
            stability: Default::default(),
        }),
    );
    command_world.imports.insert(
        WorldKey::Name("wasi:cli/stdout".to_string()),
        WorldItem::Function(Function {
            name: "stdout".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![],
            result: None,
            docs: Default::default(),
            stability: Default::default(),
        }),
    );
    command_world.exports.insert(
        WorldKey::Name("run".to_string()),
        WorldItem::Function(Function {
            name: "run".to_string(),
            kind: FunctionKind::Freestanding,
            params: vec![],
            result: None,
            docs: Default::default(),
            stability: Default::default(),
        }),
    );

    let command_world_id = resolve.worlds.alloc(command_world);
    resolve.packages[cli_package_id]
        .worlds
        .insert("command".to_string(), command_world_id);

    // Generate WIT text using wit-encoder for the second package
    let packages2 = wit_encoder::packages_from_parsed(&resolve);
    // Find the cli package (it should be the second one)
    let cli_wit_text = packages2
        .iter()
        .find(|p| p.name().namespace() == "wasi" && p.name().name().as_ref() == "cli")
        .map(|p| p.to_string())
        .unwrap_or_default();

    let interfaces = vec![
        (
            WitInterface::new_for_testing(
                1,
                Some("wasi:http@0.2.0".to_string()),
                http_wit_text,
                Some("proxy".to_string()),
                1,
                1,
                "2024-01-15T10:30:00Z".to_string(),
            ),
            "ghcr.io/example/http-proxy:v1.0.0".to_string(),
        ),
        (
            WitInterface::new_for_testing(
                2,
                Some("wasi:cli@0.2.0".to_string()),
                cli_wit_text,
                Some("command".to_string()),
                2,
                1,
                "2024-01-16T11:20:00Z".to_string(),
            ),
            "ghcr.io/example/cli-tool:latest".to_string(),
        ),
    ];
    let mut state = InterfacesViewState::new();
    let output = render_stateful_to_string(InterfacesView::new(&interfaces), &mut state, 100, 15);
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
fn test_packages_view_with_packages_snapshot() {
    let packages = vec![
        ImageEntry::new_for_testing(
            "ghcr.io".to_string(),
            "bytecode-alliance/wasmtime".to_string(),
            Some("v1.0.0".to_string()),
            Some("sha256:abc123def456".to_string()),
            1024 * 1024 * 5, // 5 MB
        ),
        ImageEntry::new_for_testing(
            "docker.io".to_string(),
            "example/hello-wasm".to_string(),
            Some("latest".to_string()),
            None,
            1024 * 512, // 512 KB
        ),
        ImageEntry::new_for_testing(
            "ghcr.io".to_string(),
            "user/my-component".to_string(),
            Some("v2.1.0".to_string()),
            Some("sha256:789xyz".to_string()),
            1024 * 1024 * 2, // 2 MB
        ),
    ];
    let output = render_to_string(PackagesView::new(&packages), 100, 15);
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

#[test]
fn test_packages_view_filter_with_results_snapshot() {
    let packages = vec![ImageEntry::new_for_testing(
        "ghcr.io".to_string(),
        "bytecode-alliance/wasi-http".to_string(),
        Some("v0.2.0".to_string()),
        Some("sha256:wasi123".to_string()),
        1024 * 256, // 256 KB
    )];
    let mut state = PackagesViewState::new();
    state.filter_query = "wasi".to_string();
    let output = render_stateful_to_string(PackagesView::new(&packages), &mut state, 100, 12);
    assert_snapshot!(output);
}

// =============================================================================
// PackageDetailView Snapshot Tests
// =============================================================================

#[test]
fn test_package_detail_view_snapshot() {
    let package = ImageEntry::new_for_testing(
        "ghcr.io".to_string(),
        "bytecode-alliance/wasmtime".to_string(),
        Some("v1.0.0".to_string()),
        Some("sha256:abc123def456789".to_string()),
        1024 * 1024 * 5, // 5 MB
    );
    let output = render_to_string(PackageDetailView::new(&package), 80, 25);
    assert_snapshot!(output);
}

#[test]
fn test_package_detail_view_without_tag_snapshot() {
    let package = ImageEntry::new_for_testing(
        "docker.io".to_string(),
        "library/hello-world".to_string(),
        None,
        Some("sha256:digest123".to_string()),
        1024 * 128, // 128 KB
    );
    let output = render_to_string(PackageDetailView::new(&package), 80, 25);
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
fn test_search_view_with_packages_snapshot() {
    let packages = vec![
        KnownPackage::new_for_testing(
            "ghcr.io".to_string(),
            "bytecode-alliance/wasi-http".to_string(),
            Some("WASI HTTP interface".to_string()),
            vec!["v0.2.0".to_string(), "v0.1.0".to_string()],
            vec![],
            vec![],
            "2024-01-15T10:30:00Z".to_string(),
            "2024-01-01T08:00:00Z".to_string(),
        ),
        KnownPackage::new_for_testing(
            "ghcr.io".to_string(),
            "user/my-component".to_string(),
            None,
            vec!["latest".to_string()],
            vec![],
            vec![],
            "2024-02-01T12:00:00Z".to_string(),
            "2024-01-20T09:00:00Z".to_string(),
        ),
    ];
    let output = render_to_string(SearchView::new(&packages), 100, 15);
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

#[test]
fn test_search_view_with_many_tags_snapshot() {
    let packages = vec![KnownPackage::new_for_testing(
        "ghcr.io".to_string(),
        "project/component".to_string(),
        Some("A component with many tags".to_string()),
        vec![
            "v3.0.0".to_string(),
            "v2.0.0".to_string(),
            "v1.0.0".to_string(),
            "beta".to_string(),
            "alpha".to_string(),
        ],
        vec!["v3.0.0.sig".to_string()],
        vec!["v3.0.0.att".to_string()],
        "2024-03-01T10:00:00Z".to_string(),
        "2023-06-01T08:00:00Z".to_string(),
    )];
    let output = render_to_string(SearchView::new(&packages), 100, 12);
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

#[test]
fn test_known_package_detail_view_minimal_snapshot() {
    let package = KnownPackage::new_for_testing(
        "docker.io".to_string(),
        "library/minimal".to_string(),
        None,
        vec!["latest".to_string()],
        vec![],
        vec![],
        "2024-02-01T12:00:00Z".to_string(),
        "2024-02-01T12:00:00Z".to_string(),
    );
    let output = render_to_string(KnownPackageDetailView::new(&package), 80, 15);
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
    let state_info = StateInfo::new_for_testing();
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
fn test_tab_bar_third_selected_snapshot() {
    let tab_bar = TabBar::new("Test App - ready", TestTab::Third);
    let output = render_to_string(tab_bar, 60, 3);
    assert_snapshot!(output);
}

#[test]
fn test_tab_bar_loading_state_snapshot() {
    let tab_bar = TabBar::new("Test App - loading...", TestTab::First);
    let output = render_to_string(tab_bar, 60, 3);
    assert_snapshot!(output);
}

#[test]
fn test_tab_bar_error_state_snapshot() {
    let tab_bar = TabBar::new("Test App - error occurred!", TestTab::First);
    let output = render_to_string(tab_bar, 60, 3);
    assert_snapshot!(output);
}
