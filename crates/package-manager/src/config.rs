//! Configuration management for wasm package manager.
//!
//! This module provides configuration support following XDG Base Directory specification.
//! Configuration is stored in JSON format for compatibility with tools like 1Password.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Main configuration structure for the wasm package manager.
///
/// Configuration is stored in JSON format at `$XDG_CONFIG_HOME/wasm/config.json`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Config {
    /// Per-registry configuration settings.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub registries: HashMap<String, RegistryConfig>,
}

impl Config {
    /// Load configuration from the XDG config directory.
    ///
    /// If the configuration file doesn't exist, a default configuration is created
    /// and written to disk.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Unable to determine the XDG config directory
    /// - Unable to create the config directory
    /// - Unable to read or parse the config file
    /// - Unable to write the default config
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            Self::load_from_path(&config_path)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Load configuration from a specific path.
    ///
    /// # Errors
    ///
    /// Returns an error if unable to read or parse the config file.
    pub fn load_from_path(path: &PathBuf) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Config = serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        Ok(config)
    }

    /// Save configuration to the XDG config directory.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Unable to determine the XDG config directory
    /// - Unable to create the config directory
    /// - Unable to write the config file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        let contents = serde_json::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&config_path, contents)
            .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

        Ok(())
    }

    /// Get the configuration file path following XDG Base Directory specification.
    ///
    /// # Errors
    ///
    /// Returns an error if unable to determine the XDG config directory.
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_local_dir()
            .context("No local config dir known for the current OS")?
            .join("wasm");

        Ok(config_dir.join("config.json"))
    }

    /// Get configuration for a specific registry.
    ///
    /// Returns `None` if no configuration exists for the registry.
    #[must_use]
    pub fn get_registry(&self, registry: &str) -> Option<&RegistryConfig> {
        self.registries.get(registry)
    }

    /// Set configuration for a specific registry.
    pub fn set_registry(&mut self, registry: String, config: RegistryConfig) {
        self.registries.insert(registry, config);
    }
}

/// Configuration for a specific registry.
///
/// This allows per-registry customization of authentication and other settings.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RegistryConfig {
    /// Custom authentication command for this registry.
    ///
    /// This can be used to integrate with tools like 1Password.
    /// The command should output credentials in a format compatible with
    /// the docker credential helper protocol.
    ///
    /// Example: `"op read op://vault/item/credential"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_command: Option<String>,

    /// Whether to use anonymous authentication for this registry.
    ///
    /// If set to `true`, no credentials will be used even if available.
    #[serde(default, skip_serializing_if = "is_false")]
    pub anonymous: bool,
}

/// Helper function for serde to skip serializing false boolean values.
fn is_false(value: &bool) -> bool {
    !value
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_config_at(dir: &TempDir) -> PathBuf {
        let config_path = dir.path().join("wasm").join("config.json");
        fs::create_dir_all(config_path.parent().expect("parent")).expect("create dir");
        config_path
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.registries.is_empty());
    }

    #[test]
    fn test_config_serialization() {
        let mut config = Config::default();

        let registry_config = RegistryConfig {
            auth_command: Some("op read op://vault/ghcr/token".to_string()),
            anonymous: false,
        };

        config.set_registry("ghcr.io".to_string(), registry_config);

        let json = serde_json::to_string_pretty(&config).expect("serialize");
        let deserialized: Config = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(config, deserialized);
        assert!(json.contains("ghcr.io"));
        assert!(json.contains("op read"));
    }

    #[test]
    fn test_config_deserialization_empty() {
        let json = "{}";
        let config: Config = serde_json::from_str(json).expect("deserialize");
        assert!(config.registries.is_empty());
    }

    #[test]
    fn test_config_with_registry() {
        let json = r#"{
            "registries": {
                "docker.io": {
                    "auth_command": "op read op://vault/docker/token"
                },
                "ghcr.io": {
                    "anonymous": true
                }
            }
        }"#;

        let config: Config = serde_json::from_str(json).expect("deserialize");

        assert_eq!(config.registries.len(), 2);

        let docker_config = config.get_registry("docker.io").expect("docker.io");
        assert_eq!(
            docker_config.auth_command,
            Some("op read op://vault/docker/token".to_string())
        );
        assert!(!docker_config.anonymous);

        let ghcr_config = config.get_registry("ghcr.io").expect("ghcr.io");
        assert_eq!(ghcr_config.auth_command, None);
        assert!(ghcr_config.anonymous);
    }

    #[test]
    fn test_config_load_and_save() {
        let temp_dir = TempDir::new().expect("temp dir");
        let config_path = create_test_config_at(&temp_dir);

        // Create a config
        let mut config = Config::default();
        config.set_registry(
            "test.registry".to_string(),
            RegistryConfig {
                auth_command: Some("test command".to_string()),
                anonymous: false,
            },
        );

        // Save it
        let json = serde_json::to_string_pretty(&config).expect("serialize");
        fs::write(&config_path, json).expect("write");

        // Load it back
        let loaded = Config::load_from_path(&config_path).expect("load");

        assert_eq!(config, loaded);
    }

    #[test]
    fn test_registry_config_default() {
        let config = RegistryConfig::default();
        assert_eq!(config.auth_command, None);
        assert!(!config.anonymous);
    }

    #[test]
    fn test_get_registry_missing() {
        let config = Config::default();
        assert!(config.get_registry("missing.registry").is_none());
    }

    #[test]
    fn test_skip_empty_registries() {
        let config = Config::default();
        let json = serde_json::to_string(&config).expect("serialize");
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_skip_false_anonymous() {
        let config = RegistryConfig {
            auth_command: Some("test".to_string()),
            anonymous: false,
        };
        let json = serde_json::to_string(&config).expect("serialize");
        assert!(!json.contains("anonymous"));
    }

    #[test]
    fn test_include_true_anonymous() {
        let config = RegistryConfig {
            auth_command: None,
            anonymous: true,
        };
        let json = serde_json::to_string(&config).expect("serialize");
        assert!(json.contains("anonymous"));
        assert!(json.contains("true"));
    }
}
