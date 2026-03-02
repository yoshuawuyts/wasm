use std::path::PathBuf;

use anyhow::{Context, Result};

use super::resolver;

/// Options for the `compose resolve` subcommand.
#[derive(clap::Args)]
pub(crate) struct ResolveOpts {
    /// Path to a `.wac` file to resolve.
    file: PathBuf,
}

impl ResolveOpts {
    pub(crate) fn run(self) -> Result<()> {
        let source = std::fs::read_to_string(&self.file)
            .with_context(|| format!("could not read '{}'", self.file.display()))?;

        let document = wac_parser::Document::parse(&source)
            .map_err(|e| anyhow::anyhow!("parse error in '{}': {e}", self.file.display()))?;

        let base = std::env::current_dir().context("could not determine current directory")?;
        let fs_resolver = resolver::build_resolver(&base)?;

        let keys = wac_resolver::packages(&document).map_err(|e| {
            anyhow::anyhow!(
                "could not determine packages in '{}': {e}",
                self.file.display()
            )
        })?;

        let packages = fs_resolver.resolve(&keys).map_err(|e| {
            anyhow::anyhow!(
                "could not resolve packages for '{}': {e}",
                self.file.display()
            )
        })?;

        let resolution = document
            .resolve(packages)
            .map_err(|e| anyhow::anyhow!("resolution error in '{}': {e}", self.file.display()))?;

        // Print a summary of the resolved document
        let doc = resolution.document();
        let json = serde_json::to_string_pretty(doc)
            .context("could not serialize resolved document to JSON")?;

        println!("{json}");
        Ok(())
    }
}
