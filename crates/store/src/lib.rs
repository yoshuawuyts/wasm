use std::path::{Path, PathBuf};

/// A cache on disk
#[derive(Debug)]
pub struct Store {
    #[allow(unused)]
    location: PathBuf,
}

impl Store {
    /// Create a new store at a location on disk.
    pub fn new(location: &Path) -> Self {
        Self {
            location: location.to_owned(),
        }
    }
}
