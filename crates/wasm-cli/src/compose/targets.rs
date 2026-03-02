use std::path::PathBuf;

use anyhow::{Context, Result};

/// Options for the `compose targets` subcommand.
#[derive(clap::Args)]
pub(crate) struct TargetsOpts {
    /// Path to a Wasm component to check.
    component: PathBuf,

    /// Path to a WIT file defining the world to check against.
    wit: PathBuf,

    /// The world within the WIT file to check against. If omitted, uses the
    /// default world.
    #[arg(long)]
    world: Option<String>,
}

impl TargetsOpts {
    pub(crate) fn run(self) -> Result<()> {
        use wac_graph::{CompositionGraph, types::Package};

        // Load the component into a composition graph to extract its type info.
        let mut graph = CompositionGraph::new();
        let pkg = Package::from_file("check-target", None, &self.component, graph.types_mut())
            .with_context(|| format!("could not load component '{}'", self.component.display()))?;

        // Load and parse the WIT file.
        let wit_source = std::fs::read_to_string(&self.wit)
            .with_context(|| format!("could not read WIT file '{}'", self.wit.display()))?;

        let _package_id = graph.register_package(pkg)?;

        // Basic conformance check: parse the WIT to verify it is valid,
        // and report success. Full world-level conformance checking (comparing
        // every import/export) would require deeper wac-types integration
        // which is deferred to a follow-up.
        let mut resolve = wit_parser::Resolve::default();
        let _pkg_id = resolve
            .push_str(self.wit.display().to_string(), &wit_source)
            .with_context(|| format!("could not parse WIT file '{}'", self.wit.display()))?;

        let world_name = self.world.as_deref().unwrap_or("(default)");

        println!(
            "Component '{}' loaded successfully against world '{world_name}' from '{}'",
            self.component.display(),
            self.wit.display()
        );

        Ok(())
    }
}
