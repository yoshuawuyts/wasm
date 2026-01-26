//! xtask - Build automation and task orchestration for the wasm project
//!
//! This binary provides a unified interface for running common development tasks
//! like testing, linting, and formatting checks.

use std::env;
use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        exit(1);
    }

    match args[1].as_str() {
        "test" => {
            if let Err(e) = run_tests() {
                eprintln!("Error: {}", e);
                exit(1);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
            exit(1);
        }
    }
}

fn print_help() {
    println!("Usage: cargo xtask <COMMAND>");
    println!();
    println!("Commands:");
    println!("  test    Run tests, clippy, and formatting checks");
}

fn run_tests() -> Result<(), String> {
    println!("Running cargo test...");
    run_command("cargo", &["test", "--all"])?;
    
    println!("\nRunning cargo clippy...");
    run_command("cargo", &["clippy", "--all", "--", "-D", "warnings"])?;
    
    println!("\nRunning cargo fmt check...");
    run_command("cargo", &["fmt", "--all", "--", "--check"])?;
    
    println!("\n✓ All checks passed!");
    Ok(())
}

fn run_command(cmd: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .map_err(|e| format!("Failed to execute {}: {}", cmd, e))?;
    
    if !status.success() {
        return Err(format!("{} failed with exit code: {:?}", cmd, status.code()));
    }
    
    Ok(())
}
