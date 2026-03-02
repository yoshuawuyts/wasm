#![allow(clippy::print_stdout, clippy::print_stderr)]

mod build;
mod parse;
mod plug;
mod resolve;
mod resolver;
mod targets;

/// Compose and manage sets of interdependent Wasm components
#[derive(clap::Parser)]
pub(crate) enum Opts {
    /// Build a composed component from a WAC script
    Build(build::BuildOpts),
    /// Plug component exports into a socket component's imports
    Plug(plug::PlugOpts),
    /// Check if a component conforms to a WIT world
    Targets(targets::TargetsOpts),
    /// Parse a WAC file and print its AST as JSON
    Parse(parse::ParseOpts),
    /// Parse and resolve a WAC file and print the resolved representation as JSON
    Resolve(resolve::ResolveOpts),
}

impl Opts {
    pub(crate) fn run(self) -> anyhow::Result<()> {
        match self {
            Opts::Build(opts) => opts.run(),
            Opts::Plug(opts) => opts.run(),
            Opts::Targets(opts) => opts.run(),
            Opts::Parse(opts) => opts.run(),
            Opts::Resolve(opts) => opts.run(),
        }
    }
}
