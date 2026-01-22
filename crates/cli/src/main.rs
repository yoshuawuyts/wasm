//! Wasm CLI command
//!

mod inspect;
mod package;

use clap::Parser;

#[derive(clap::Parser)]
enum Command {
    /// Execute a Wasm Component
    Run,
    /// Inspect a Wasm Component
    Inspect(inspect::Opts),
    /// Convert a Wasm Component to another format
    Convert,
    /// Package, push, and pull Wasm Components
    #[command(subcommand)]
    Package(package::Opts),
    /// Compose Wasm Components with other components
    Compose,
    /// Configure the `wasm(1)` tool, generate completions, & manage state
    #[clap(name = "self")]
    Self_,
}

impl Command {
    fn run(&mut self) -> Result<(), anyhow::Error> {
        match self {
            Command::Run => todo!(),
            Command::Inspect(opts) => opts.run()?,
            Command::Convert => todo!(),
            Command::Package(opts) => opts.run()?,
            Command::Compose => todo!(),
            Command::Self_ => todo!(),
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let mut command = Command::parse();
    command.run()?;
    Ok(())
}
