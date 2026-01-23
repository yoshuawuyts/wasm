use anyhow::Result;
use wasm_package_manager::{Manager, Reference};

/// Package, push, and pull Wasm Components
#[derive(clap::Parser)]
pub(crate) enum Opts {
    /// Fetch OCI metadata for a component
    Show,
    /// Pull a component from the registry
    Pull(PullOpts),
    Push,
}

#[derive(clap::Args)]
pub(crate) struct PullOpts {
    /// The reference to pull
    reference: Reference,
}

impl Opts {
    pub(crate) async fn run(self) -> Result<()> {
        let store = Manager::open().await?;
        match self {
            Opts::Show => todo!(),
            Opts::Pull(opts) => {
                store.pull(opts.reference).await?;
                Ok(())
            }
            Opts::Push => todo!(),
        }
    }
}
