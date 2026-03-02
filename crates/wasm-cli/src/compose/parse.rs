use std::path::PathBuf;

use anyhow::{Context, Result};

/// Options for the `compose parse` subcommand.
#[derive(clap::Args)]
pub(crate) struct ParseOpts {
    /// Path to a `.wac` file to parse.
    file: PathBuf,
}

impl ParseOpts {
    pub(crate) fn run(self) -> Result<()> {
        let source = std::fs::read_to_string(&self.file)
            .with_context(|| format!("could not read '{}'", self.file.display()))?;

        let document = wac_parser::Document::parse(&source)
            .map_err(|e| anyhow::anyhow!("parse error in '{}': {e}", self.file.display()))?;

        let json =
            serde_json::to_string_pretty(&document).context("could not serialize AST to JSON")?;

        println!("{json}");
        Ok(())
    }
}
