//! Global configuration module for the package manager.
//!
//! This module provides support for reading and managing a global TOML configuration
//! file at `$XDG_CONFIG_HOME/wasm/config.toml`. The configuration file supports
//! per-registry credential helpers for secure authentication.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::RwLock;

/// Default configuration file content with commented examples.
const DEFAULT_CONFIG: &str = r#"# wasm(1) configuration file
# https://github.com/yoshuawuyts/wasm

# Per-registry credential helpers allow secure authentication with container registries.
# Credentials are fetched on-demand and never stored to disk.

# Example configurations (uncomment and modify as needed):

# Option 1: Single JSON command (recommended for 1Password)
# The command should output JSON with username and password fields:
# [{"id": "username", "value": "..."}, {"id": "password", "value": "..."}]
#
# [registries."ghcr.io"]
# credential-helper = "op item get ghcr --format json --fields username,password"

# Option 2: Two separate commands (for simple scripts)
#
# [registries."my-registry.example.com"]
# credential-helper.username = "/path/to/get-user.sh"
# credential-helper.password = "/path/to/get-pass.sh"
"#;

/// The main configuration struct.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Default registry to use when no registry is specified.
    #[serde(rename = "default-registry")]
    pub default_registry: Option<String>,

    /// Per-registry configuration.
    #[serde(default)]
    pub registries: HashMap<String, RegistryConfig>,

    /// Runtime credential cache (not serialized).
    #[serde(skip)]
    credential_cache: CredentialCache,
}

/// Thread-safe credential cache.
#[derive(Debug, Default)]
struct CredentialCache {
    cache: RwLock<HashMap<String, (String, String)>>,
}

impl Clone for CredentialCache {
    fn clone(&self) -> Self {
        let cache = self.cache.read().expect("Failed to acquire read lock");
        Self {
            cache: RwLock::new(cache.clone()),
        }
    }
}

/// Configuration for a specific registry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RegistryConfig {
    /// Credential helper configuration for this registry.
    #[serde(rename = "credential-helper")]
    pub credential_helper: Option<CredentialHelper>,
}

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

/// A field from the JSON credential helper output.
#[derive(Debug, Deserialize)]
struct CredentialField {
    id: String,
    value: String,
}

impl Config {
    /// Load configuration from the default config directory.
    ///
    /// The configuration file is expected at `$XDG_CONFIG_HOME/wasm/config.toml`.
    /// If the file doesn't exist, returns a default configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration file exists but cannot be read or parsed.
    pub fn load() -> Result<Self> {
        Self::load_from(None)
    }

    /// Load configuration from a specified directory (for testing).
    ///
    /// If `config_dir` is `None`, uses the default XDG config directory.
    /// If the file doesn't exist, returns a default configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration file exists but cannot be read or parsed.
    pub fn load_from(config_dir: Option<PathBuf>) -> Result<Self> {
        let config_path = Self::config_path_from(config_dir);
        Self::load_from_path(&config_path)
    }

    /// Load configuration from a specific file path.
    ///
    /// If the file doesn't exist, returns a default configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration file exists but cannot be read or parsed.
    pub fn load_from_path(config_path: &Path) -> Result<Self> {
        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?;

        Ok(config)
    }

    /// Returns the path to the configuration file.
    #[must_use]
    pub fn config_path() -> PathBuf {
        Self::config_path_from(None)
    }

    /// Returns the path to the configuration file from a specified directory.
    #[must_use]
    pub fn config_path_from(config_dir: Option<PathBuf>) -> PathBuf {
        let base = config_dir
            .unwrap_or_else(|| dirs::config_dir().unwrap_or_else(|| PathBuf::from(".config")));
        base.join("wasm").join("config.toml")
    }

    /// Ensures the configuration file exists, creating a default one if not.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory or file cannot be created.
    pub fn ensure_exists() -> Result<PathBuf> {
        Self::ensure_exists_at(None)
    }

    /// Ensures the configuration file exists at a specified directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory or file cannot be created.
    pub fn ensure_exists_at(config_dir: Option<PathBuf>) -> Result<PathBuf> {
        let config_path = Self::config_path_from(config_dir);

        if config_path.exists() {
            return Ok(config_path);
        }

        // Create parent directory if needed
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        // Write default configuration
        fs::write(&config_path, DEFAULT_CONFIG).with_context(|| {
            format!(
                "Failed to write default config file: {}",
                config_path.display()
            )
        })?;

        Ok(config_path)
    }

    /// Get credentials for a registry using the configured credential helper.
    ///
    /// Returns `None` if no credential helper is configured for the registry.
    /// Results are cached in memory for subsequent calls.
    ///
    /// # Errors
    ///
    /// Returns an error if the credential helper command fails or returns invalid output.
    pub fn get_credentials(&self, registry: &str) -> Result<Option<(String, String)>> {
        // Check cache first
        {
            let cache = self
                .credential_cache
                .cache
                .read()
                .expect("Failed to acquire read lock");
            if let Some(creds) = cache.get(registry) {
                return Ok(Some(creds.clone()));
            }
        }

        // Look up registry config
        let registry_config = match self.registries.get(registry) {
            Some(config) => config,
            None => return Ok(None),
        };

        // Check if credential helper is configured
        let helper = match &registry_config.credential_helper {
            Some(h) => h,
            None => return Ok(None),
        };

        // Execute credential helper
        let credentials = match helper {
            CredentialHelper::Json(cmd) => execute_json_helper(cmd)?,
            CredentialHelper::Split { username, password } => {
                execute_split_helper(username, password)?
            }
        };

        // Cache the result
        {
            let mut cache = self
                .credential_cache
                .cache
                .write()
                .expect("Failed to acquire write lock");
            cache.insert(registry.to_string(), credentials.clone());
        }

        Ok(Some(credentials))
    }

    /// Clear the credential cache.
    pub fn clear_credential_cache(&self) {
        let mut cache = self
            .credential_cache
            .cache
            .write()
            .expect("Failed to acquire write lock");
        cache.clear();
    }
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

    let fields: Vec<CredentialField> = serde_json::from_str(&output)
        .with_context(|| format!("Failed to parse credential helper output as JSON: {output}"))?;

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
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.default_registry.is_none());
        assert!(config.registries.is_empty());
    }

    #[test]
    fn test_config_load_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let config = Config::load_from(Some(temp_dir.path().to_path_buf())).unwrap();
        assert!(config.default_registry.is_none());
        assert!(config.registries.is_empty());
    }

    #[test]
    fn test_config_load_valid() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("wasm");
        fs::create_dir_all(&config_dir).unwrap();

        let config_path = config_dir.join("config.toml");
        let toml_content = r#"
default-registry = "ghcr.io"

[registries."ghcr.io"]
credential-helper = "echo test"
"#;
        fs::write(&config_path, toml_content).unwrap();

        let config = Config::load_from(Some(temp_dir.path().to_path_buf())).unwrap();
        assert_eq!(config.default_registry, Some("ghcr.io".to_string()));
        assert!(config.registries.contains_key("ghcr.io"));
    }

    #[test]
    fn test_config_load_split_helper() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("wasm");
        fs::create_dir_all(&config_dir).unwrap();

        let config_path = config_dir.join("config.toml");
        let toml_content = r#"
[registries."my-registry.example.com"]
credential-helper.username = "/path/to/get-user.sh"
credential-helper.password = "/path/to/get-pass.sh"
"#;
        fs::write(&config_path, toml_content).unwrap();

        let config = Config::load_from(Some(temp_dir.path().to_path_buf())).unwrap();
        let registry_config = config.registries.get("my-registry.example.com").unwrap();

        match &registry_config.credential_helper {
            Some(CredentialHelper::Split { username, password }) => {
                assert_eq!(username, "/path/to/get-user.sh");
                assert_eq!(password, "/path/to/get-pass.sh");
            }
            _ => panic!("Expected Split credential helper"),
        }
    }

    #[test]
    fn test_config_ensure_exists() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = Config::ensure_exists_at(Some(temp_dir.path().to_path_buf())).unwrap();

        assert!(config_path.exists());

        let content = fs::read_to_string(&config_path).unwrap();
        assert!(content.contains("credential-helper"));
    }

    #[test]
    fn test_config_ensure_exists_idempotent() {
        let temp_dir = TempDir::new().unwrap();

        // First call creates the file
        let path1 = Config::ensure_exists_at(Some(temp_dir.path().to_path_buf())).unwrap();

        // Modify the file
        let mut file = fs::OpenOptions::new().append(true).open(&path1).unwrap();
        writeln!(file, "# custom comment").unwrap();

        // Second call should not overwrite
        let path2 = Config::ensure_exists_at(Some(temp_dir.path().to_path_buf())).unwrap();
        assert_eq!(path1, path2);

        let content = fs::read_to_string(&path2).unwrap();
        assert!(content.contains("# custom comment"));
    }

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
    fn test_credential_cache() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("wasm");
        fs::create_dir_all(&config_dir).unwrap();

        let config_path = config_dir.join("config.toml");
        let toml_content = r#"
[registries."test.io"]
credential-helper = "echo '[{\"id\": \"username\", \"value\": \"user\"}, {\"id\": \"password\", \"value\": \"pass\"}]'"
"#;
        fs::write(&config_path, toml_content).unwrap();

        let config = Config::load_from(Some(temp_dir.path().to_path_buf())).unwrap();

        // First call should execute the helper
        let creds = config.get_credentials("test.io").unwrap();
        assert_eq!(creds, Some(("user".to_string(), "pass".to_string())));

        // Clear cache
        config.clear_credential_cache();

        // After clearing, no cached entry
        let cache = config.credential_cache.cache.read().unwrap();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_get_credentials_no_helper() {
        let config = Config::default();
        let creds = config.get_credentials("unknown.io").unwrap();
        assert!(creds.is_none());
    }
}
