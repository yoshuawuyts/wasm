use std::path::PathBuf;

use anyhow::{Context, Result};
use wac_graph::{CompositionGraph, EncodeOptions, plug, types::Package};

/// Options for the `compose plug` subcommand.
#[derive(clap::Args)]
pub(crate) struct PlugOpts {
    /// Path to the socket component (the component that receives imports).
    socket: PathBuf,

    /// Paths to plug components whose exports fill the socket's imports.
    #[arg(long = "plug", required = true)]
    plugs: Vec<PathBuf>,

    /// Output path for the composed component.
    #[arg(short, long, default_value = "plugged.wasm")]
    output: PathBuf,
}

impl PlugOpts {
    pub(crate) fn run(self) -> Result<()> {
        let mut graph = CompositionGraph::new();

        // Register socket
        let socket_pkg = Package::from_file("socket", None, &self.socket, graph.types_mut())
            .with_context(|| {
                format!(
                    "could not load socket component '{}'",
                    self.socket.display()
                )
            })?;
        let socket_id = graph.register_package(socket_pkg)?;

        // Register plugs
        let mut plug_ids = Vec::new();
        for (idx, plug_path) in self.plugs.iter().enumerate() {
            let name = format!("plug-{idx}");
            let plug_pkg = Package::from_file(&name, None, plug_path, graph.types_mut())
                .with_context(|| {
                    format!("could not load plug component '{}'", plug_path.display())
                })?;
            let plug_id = graph.register_package(plug_pkg)?;
            plug_ids.push(plug_id);
        }

        // Perform plug composition
        plug(&mut graph, plug_ids, socket_id)?;

        // Encode output
        let bytes = graph.encode(EncodeOptions::default())?;

        if let Some(parent) = self.output.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&self.output, bytes)
            .with_context(|| format!("could not write '{}'", self.output.display()))?;

        println!("Plugged component written to {}", self.output.display());
        Ok(())
    }
}
