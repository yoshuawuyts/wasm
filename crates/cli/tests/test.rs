//! Tests for the wasm CLI
//!
//! This module contains integration tests for CLI commands.
//! Use `cargo test --package wasm --test test` to run these tests.
//!
//! # CLI Help Screen Tests
//!
//! These tests verify that CLI help screens remain consistent using snapshot testing.
//! When commands change, update snapshots with:
//! `cargo insta review` or `INSTA_UPDATE=always cargo test --package wasm`

use std::process::Command;

use insta::assert_snapshot;

/// Run the CLI with the given arguments and capture the output.
///
/// The output is normalized to replace platform-specific binary names
/// (e.g., `wasm.exe` on Windows) with `wasm` for consistent snapshots.
fn run_cli(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_wasm"))
        .args(args)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Combine stdout and stderr for help output (clap writes to stdout by default for --help)
    let result = if !stdout.is_empty() {
        stdout.to_string()
    } else {
        stderr.to_string()
    };

    // Normalize binary name for cross-platform consistency
    // On Windows, the binary is "wasm.exe" but on Unix it's "wasm"
    result.replace("wasm.exe", "wasm")
}

// =============================================================================
// Main CLI Help Tests
// =============================================================================

#[test]
fn test_cli_main_help_snapshot() {
    let output = run_cli(&["--help"]);
    assert_snapshot!(output);
}

#[test]
fn test_cli_version_snapshot() {
    let output = run_cli(&["--version"]);
    // Version may change, so we just verify the format
    assert!(output.contains("wasm"));
}

// =============================================================================
// Inspect Command Help Tests
// =============================================================================

#[test]
fn test_cli_inspect_help_snapshot() {
    let output = run_cli(&["inspect", "--help"]);
    assert_snapshot!(output);
}

// =============================================================================
// Local Command Help Tests
// =============================================================================

#[test]
fn test_cli_local_help_snapshot() {
    let output = run_cli(&["local", "--help"]);
    assert_snapshot!(output);
}

#[test]
fn test_cli_local_list_help_snapshot() {
    let output = run_cli(&["local", "list", "--help"]);
    assert_snapshot!(output);
}

// =============================================================================
// Package Command Help Tests
// =============================================================================

#[test]
fn test_cli_package_help_snapshot() {
    let output = run_cli(&["package", "--help"]);
    assert_snapshot!(output);
}

#[test]
fn test_cli_package_pull_help_snapshot() {
    let output = run_cli(&["package", "pull", "--help"]);
    assert_snapshot!(output);
}

#[test]
fn test_cli_package_tags_help_snapshot() {
    let output = run_cli(&["package", "tags", "--help"]);
    assert_snapshot!(output);
}

// =============================================================================
// Self Command Help Tests
// =============================================================================

#[test]
fn test_cli_self_help_snapshot() {
    let output = run_cli(&["self", "--help"]);
    assert_snapshot!(output);
}

#[test]
fn test_cli_self_state_help_snapshot() {
    let output = run_cli(&["self", "state", "--help"]);
    assert_snapshot!(output);
}
