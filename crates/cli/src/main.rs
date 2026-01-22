//! Wasm CLI command
//!

mod core;
mod inspect;
mod package;
mod self_;

use clap::Parser;

#[derive(clap::Parser)]
enum Command {
    /// Execute a Wasm Component
    #[command(subcommand)]
    Run,
    /// Inspect a Wasm Component
    Inspect(inspect::Opts),
    /// Convert a Wasm Component to another format
    #[command(subcommand)]
    Convert,
    /// Package, push, and pull Wasm Components
    #[command(subcommand)]
    Package(package::Opts),
    /// Compose Wasm Components with other components
    #[command(subcommand)]
    Compose,
    /// Configure the `wasm(1)` tool, generate completions, & manage state
    #[clap(name = "self")]
    #[command(subcommand)]
    Self_(self_::Opts),
}

impl Command {
    async fn run(self) -> Result<(), anyhow::Error> {
        match self {
            Command::Run => todo!(),
            Command::Inspect(opts) => opts.run()?,
            Command::Convert => todo!(),
            Command::Package(opts) => opts.run().await?,
            Command::Compose => todo!(),
            Command::Self_(opts) => opts.run()?,
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let command = Command::parse();
    command.run().await?;
    Ok(())
}
