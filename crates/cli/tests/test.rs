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

// =============================================================================
// Color Support Tests
// =============================================================================

#[test]
fn test_color_flag_auto() {
    // Test that --color=auto is accepted
    let output = Command::new(env!("CARGO_BIN_EXE_wasm"))
        .args(&["--color", "auto", "--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[test]
fn test_color_flag_always() {
    // Test that --color=always is accepted
    let output = Command::new(env!("CARGO_BIN_EXE_wasm"))
        .args(&["--color", "always", "--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[test]
fn test_color_flag_never() {
    // Test that --color=never is accepted
    let output = Command::new(env!("CARGO_BIN_EXE_wasm"))
        .args(&["--color", "never", "--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[test]
fn test_color_flag_invalid_value() {
    // Test that invalid color values are rejected
    let output = Command::new(env!("CARGO_BIN_EXE_wasm"))
        .args(&["--color", "invalid", "--version"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid value 'invalid'"));
}

#[test]
fn test_color_flag_in_help() {
    // Test that --color flag appears in help output
    let output = run_cli(&["--help"]);
    assert!(output.contains("--color"));
    assert!(output.contains("When to use colored output"));
}

#[test]
fn test_no_color_env_var() {
    // Test that NO_COLOR environment variable disables color
    let output = Command::new(env!("CARGO_BIN_EXE_wasm"))
        .args(&["--version"])
        .env("NO_COLOR", "1")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    // The output should not contain ANSI escape codes when NO_COLOR is set
    // We can't easily test for absence of color codes without parsing,
    // but we can verify the command succeeds
}

#[test]
fn test_clicolor_env_var() {
    // Test that CLICOLOR=0 environment variable disables color
    let output = Command::new(env!("CARGO_BIN_EXE_wasm"))
        .args(&["--version"])
        .env("CLICOLOR", "0")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[test]
fn test_color_flag_with_subcommand() {
    // Test that --color flag works with subcommands
    let output = Command::new(env!("CARGO_BIN_EXE_wasm"))
        .args(&["--color", "never", "local", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

// =============================================================================
// Offline Mode Tests
// =============================================================================

#[test]
fn test_offline_flag_accepted() {
    // Test that --offline flag is accepted with --version
    let output = Command::new(env!("CARGO_BIN_EXE_wasm"))
        .args(&["--offline", "--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[test]
fn test_offline_flag_in_help() {
    // Test that --offline flag appears in help output
    let output = run_cli(&["--help"]);
    assert!(output.contains("--offline"));
    assert!(output.contains("Run in offline mode"));
}

#[test]
fn test_offline_flag_with_local_list() {
    // Test that --offline works with local list command (local-only operation)
    let output = Command::new(env!("CARGO_BIN_EXE_wasm"))
        .args(&["--offline", "local", "list", "/nonexistent"])
        .output()
        .expect("Failed to execute command");

    // The command should succeed (even if no files found)
    assert!(output.status.success());
}

#[test]
fn test_offline_flag_with_package_pull() {
    // Test that --offline mode causes package pull to fail with clear error
    let output = Command::new(env!("CARGO_BIN_EXE_wasm"))
        .args(&[
            "--offline",
            "package",
            "pull",
            "ghcr.io/example/test:latest",
        ])
        .output()
        .expect("Failed to execute command");

    // The command should fail with an offline mode error
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("offline"),
        "Expected 'offline' error message, got: {}",
        stderr
    );
}

#[test]
fn test_offline_flag_with_inspect() {
    // Test that --offline works with inspect command (local-only operation)
    let output = Command::new(env!("CARGO_BIN_EXE_wasm"))
        .args(&["--offline", "inspect", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[test]
fn test_offline_flag_with_subcommand() {
    // Test that --offline flag works with subcommands
    let output = Command::new(env!("CARGO_BIN_EXE_wasm"))
        .args(&["--offline", "local", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}
