use anyhow::Result;
use wasm_package_manager::{Config, Manager, format_size};

/// Configure the `wasm(1)` tool, generate completions, & manage state
#[derive(clap::Parser)]
pub(crate) enum Opts {
    /// Print diagnostics about the local state
    State,
    /// Show configuration file location and current settings
    Config,
}

impl Opts {
    pub(crate) async fn run(&self) -> Result<()> {
        match self {
            Opts::State => {
                let store = Manager::open().await?;
                let state_info = store.state_info();

                println!("[Migrations]");
                println!(
                    "Current: \t{}/{}",
                    state_info.migration_current(),
                    state_info.migration_total()
                );
                println!();
                println!("[Storage]");
                println!("Executable: \t{}", state_info.executable().display());
                println!("Data storage: \t{}", state_info.data_dir().display());
                println!(
                    "Content store: \t{} ({})",
                    state_info.store_dir().display(),
                    format_size(state_info.store_size())
                );
                println!(
                    "Image metadata: {} ({})",
                    state_info.metadata_file().display(),
                    format_size(state_info.metadata_size())
                );
                Ok(())
            }
            Opts::Config => {
                // Get the config path
                let config_path = Config::config_path();

                println!("[Configuration]");
                println!("Config file:\t{}", config_path.display());

                // Check if the config file exists
                if config_path.exists() {
                    println!("Status:\t\texists");
                } else {
                    println!("Status:\t\tnot created (will use defaults)");
                    println!();
                    println!("To create a default config file with examples, run:");
                    println!("  mkdir -p {}", config_path.parent().unwrap().display());
                    println!(
                        "  echo '' >> {}  # or run 'wasm self config' after creating it",
                        config_path.display()
                    );
                }

                // Load the config to show current settings
                let config = Config::load()?;
                println!();
                println!("[Settings]");

                // Show default registry if set
                if let Some(ref registry) = config.default_registry {
                    println!("Default registry:\t{registry}");
                } else {
                    println!("Default registry:\t(not set)");
                }

                // Show configured registries
                if config.registries.is_empty() {
                    println!("Registries:\t\t(none configured)");
                } else {
                    println!("Registries:");
                    for (name, registry_config) in &config.registries {
                        let helper_status = if registry_config.credential_helper.is_some() {
                            "credential-helper configured"
                        } else {
                            "no credential-helper"
                        };
                        println!("  - {name}: {helper_status}");
                    }
                }

                Ok(())
            }
        }
    }
}
