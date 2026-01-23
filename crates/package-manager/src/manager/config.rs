use anyhow::Context;
use std::path::{Path, PathBuf};

/// Information about the store
#[derive(Debug)]
pub struct Config {
    location: PathBuf,
    layers_dir: PathBuf,
    metadata_dir: PathBuf,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let location = dirs::data_local_dir()
            .context("No local data dir known for the current OS")?
            .join("wasm");
        Ok(Self::new_at(location))
    }

    pub fn new_at(location: PathBuf) -> Self {
        Self {
            layers_dir: location.join("layers"),
            metadata_dir: location.join("metadata"),
            location,
        }
    }
    /// Get the location of the crate's data dir
    pub fn data_dir(&self) -> &Path {
        &self.location
    }

    /// Get the location of the crate's cache dir
    pub fn layers_dir(&self) -> &Path {
        &self.layers_dir
    }

    /// Get the location of the crate's metadata dir
    pub fn metadata_dir(&self) -> &Path {
        &self.metadata_dir
    }
}
