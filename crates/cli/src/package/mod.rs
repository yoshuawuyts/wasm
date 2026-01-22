use anyhow::Result;

/// Read metadata (module name, producers) from a WebAssembly file.
#[derive(clap::Parser)]
pub(crate) enum Opts {
    /// Fetch OCI metadata for a component
    Show,
    Pull,
    Push,
}

impl Opts {
    pub(crate) fn run(&self) -> Result<()> {
        Ok(())
    }
}
