use std::env;

use anyhow::Result;
use wasm_package_manager::Manager;

/// Configure the `wasm(1)` tool, generate completions, & manage state
#[derive(clap::Parser)]
pub(crate) enum Opts {
    /// Print diagnostics about the local state
    State,
}

impl Opts {
    pub(crate) async fn run(&self) -> Result<()> {
        match self {
            Opts::State => {
                let store = Manager::open().await?;
                println!("[Storage]");
                println!("Executable: \t{}", executable_dir());
                println!(
                    "Data storage: \t{}",
                    store.config().data_dir().to_string_lossy()
                );
                println!(
                    "Image layers: \t{}",
                    store.config().layers_dir().to_string_lossy()
                );
                println!(
                    "Image metadata: {}",
                    store.config().metadata_dir().to_string_lossy()
                );
                Ok(())
            }
        }
    }
}

/// Get the location of the current executable
fn executable_dir() -> String {
    match env::current_exe() {
        Ok(exe_path) => exe_path.display().to_string(),
        Err(_) => String::from("unknown executable dir"),
    }
}
