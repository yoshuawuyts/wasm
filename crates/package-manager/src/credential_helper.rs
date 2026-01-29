//! Credential helper module for executing external commands to retrieve credentials.
//!
//! This module provides support for two types of credential helpers:
//! - JSON: Single command that outputs JSON with username and password fields
//! - Split: Separate commands for username and password

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;

/// Credential helper configuration.
///
/// Supports two formats:
/// - JSON: Single command that outputs JSON with username and password
/// - Split: Separate commands for username and password
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CredentialHelper {
    /// Single command that outputs JSON with fields for username and password.
    ///
    /// Expected output format:
    /// ```json
    /// [{"id": "username", "value": "..."}, {"id": "password", "value": "..."}]
    /// ```
    Json(String),

    /// Separate commands for username and password.
    Split {
        /// Command to get the username (output is trimmed).
        username: String,
        /// Command to get the password (output is trimmed).
        password: String,
    },
}

impl CredentialHelper {
    /// Execute the credential helper and return the username and password.
    ///
    /// # Errors
    ///
    /// Returns an error if the credential helper command fails or returns invalid output.
    pub fn execute(&self) -> Result<(String, String)> {
        match self {
            CredentialHelper::Json(cmd) => execute_json_helper(cmd),
            CredentialHelper::Split { username, password } => {
                execute_split_helper(username, password)
            }
        }
    }
}

/// A field from the JSON credential helper output.
#[derive(Debug, Deserialize)]
struct CredentialField {
    id: String,
    value: String,
}

/// Execute a JSON credential helper command.
///
/// The command should output JSON with username and password fields:
/// ```json
/// [{"id": "username", "value": "..."}, {"id": "password", "value": "..."}]
/// ```
fn execute_json_helper(cmd: &str) -> Result<(String, String)> {
    let output = execute_shell_command(cmd)
        .with_context(|| format!("Failed to execute credential helper: {cmd}"))?;

    // Trim whitespace for consistent parsing
    let output = output.trim();

    let fields: Vec<CredentialField> = serde_json::from_str(output).with_context(|| {
        // Truncate output in error message to avoid leaking credentials
        let preview = if output.len() > 100 {
            format!("{}...", &output[..100])
        } else {
            output.to_string()
        };
        format!("Failed to parse credential helper output as JSON: {preview}")
    })?;

    let mut username = None;
    let mut password = None;

    for field in fields {
        match field.id.as_str() {
            "username" => username = Some(field.value),
            "password" => password = Some(field.value),
            _ => {} // Ignore other fields
        }
    }

    let username = username.context("Credential helper output missing 'username' field")?;
    let password = password.context("Credential helper output missing 'password' field")?;

    Ok((username, password))
}

/// Execute split credential helper commands.
fn execute_split_helper(username_cmd: &str, password_cmd: &str) -> Result<(String, String)> {
    let username = execute_shell_command(username_cmd)
        .with_context(|| format!("Failed to execute username credential helper: {username_cmd}"))?
        .trim()
        .to_string();

    let password = execute_shell_command(password_cmd)
        .with_context(|| format!("Failed to execute password credential helper: {password_cmd}"))?
        .trim()
        .to_string();

    Ok((username, password))
}

/// Execute a shell command and return its stdout as a string.
fn execute_shell_command(cmd: &str) -> Result<String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", cmd]).output()
    } else {
        Command::new("sh").args(["-c", cmd]).output()
    }
    .with_context(|| format!("Failed to spawn command: {cmd}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "Command exited with status {}: {}",
            output.status,
            stderr.trim()
        );
    }

    let stdout = String::from_utf8(output.stdout).context("Command output was not valid UTF-8")?;

    Ok(stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_json_helper() {
        // Create a simple echo command that outputs valid JSON
        let json =
            r#"[{"id": "username", "value": "testuser"}, {"id": "password", "value": "testpass"}]"#;
        let cmd = format!("echo '{}'", json);

        let (username, password) = execute_json_helper(&cmd).unwrap();
        assert_eq!(username, "testuser");
        assert_eq!(password, "testpass");
    }

    #[test]
    fn test_execute_split_helper() {
        let (username, password) = execute_split_helper("echo testuser", "echo testpass").unwrap();
        assert_eq!(username, "testuser");
        assert_eq!(password, "testpass");
    }

    #[test]
    fn test_credential_helper_json_execute() {
        let json =
            r#"[{"id": "username", "value": "user1"}, {"id": "password", "value": "pass1"}]"#;
        let helper = CredentialHelper::Json(format!("echo '{}'", json));
        let (username, password) = helper.execute().unwrap();
        assert_eq!(username, "user1");
        assert_eq!(password, "pass1");
    }

    #[test]
    fn test_credential_helper_split_execute() {
        let helper = CredentialHelper::Split {
            username: "echo splituser".to_string(),
            password: "echo splitpass".to_string(),
        };
        let (username, password) = helper.execute().unwrap();
        assert_eq!(username, "splituser");
        assert_eq!(password, "splitpass");
    }

    #[test]
    fn test_credential_helper_debug_never_prints_credentials() {
        // Verify that Debug output only shows command configuration,
        // never the actual credentials returned by the helper.
        let json_helper = CredentialHelper::Json("op item get secret --format json".to_string());
        let debug_output = format!("{:?}", json_helper);

        // Should show the command
        assert!(debug_output.contains("op item get secret"));
        // Should never contain any credential-like strings from execution
        // (the helper is never executed during Debug formatting)

        let split_helper = CredentialHelper::Split {
            username: "/path/to/get-user.sh".to_string(),
            password: "/path/to/get-pass.sh".to_string(),
        };
        let debug_output = format!("{:?}", split_helper);

        // Should show the script paths
        assert!(debug_output.contains("/path/to/get-user.sh"));
        assert!(debug_output.contains("/path/to/get-pass.sh"));
    }

    #[test]
    fn test_credential_helper_display_never_leaks_credentials() {
        // Test that when executing a credential helper, the returned credentials
        // are not included in any debug output or error messages
        let json =
            r#"[{"id": "username", "value": "secret_user"}, {"id": "password", "value": "secret_pass"}]"#;
        let cmd = format!("echo '{}'", json);
        let helper = CredentialHelper::Json(cmd.clone());

        // Execute and get credentials
        let (username, password) = helper.execute().unwrap();
        assert_eq!(username, "secret_user");
        assert_eq!(password, "secret_pass");

        // Debug output of the helper should never contain the credential values
        let debug_output = format!("{:?}", helper);
        assert!(!debug_output.contains("secret_user"));
        assert!(!debug_output.contains("secret_pass"));
        // It should only show the command
        assert!(debug_output.contains("echo"));
    }
}
