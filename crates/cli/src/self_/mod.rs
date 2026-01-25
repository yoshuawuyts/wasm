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
                let state_info = store.state_info();

                println!("[Migrations]");
                println!(
                    "Current: \t{}/{}",
                    state_info.migration_current(), state_info.migration_total()
                );
                println!();
                println!("[Storage]");
                println!("Executable: \t{}", state_info.executable().display());
                println!("Data storage: \t{}", state_info.data_dir().display());
                println!("Image layers: \t{}", state_info.layers_dir().display());
                println!("Image metadata: {}", state_info.metadata_file().display());
                Ok(())
            }
        }
    }
}
