use crate::core::dirs;
use anyhow::Result;

/// Configure the `wasm(1)` tool, generate completions, & manage state
#[derive(clap::Parser)]
pub(crate) enum Opts {
    /// Print diagnostics about the local state
    State,
}

impl Opts {
    pub(crate) fn run(&self) -> Result<()> {
        match self {
            Opts::State => {
                println!("[Locations]");
                println!("Data: \t\t{}", dirs::data_dir().to_string_lossy());
                println!("Artifacts: \t{}", dirs::artifact_dir().to_string_lossy());
                println!("Executable: \t{}", dirs::executable_dir());
                Ok(())
            }
        }
    }
}
