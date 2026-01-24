//! Wasm CLI command
//!

mod inspect;
mod package;
mod self_;
mod tui;

use std::io::IsTerminal;

use clap::{CommandFactory, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Cli {
    async fn run(self) -> Result<(), anyhow::Error> {
        match self.command {
            Some(Command::Run) => todo!(),
            Some(Command::Inspect(opts)) => opts.run()?,
            Some(Command::Convert) => todo!(),
            Some(Command::Package(opts)) => opts.run().await?,
            Some(Command::Compose) => todo!(),
            Some(Command::Self_(opts)) => opts.run().await?,
            None if std::io::stdin().is_terminal() => tui::run().await?,
            None => Cli::command().print_help()?,
        }
        Ok(())
    }
}

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Cli::parse().run().await?;
    Ok(())
}
