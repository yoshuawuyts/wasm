//! A stateful library to interact with OCI registries storing WebAssembly Components.

mod manager;
mod network;
mod storage;

pub use manager::Manager;
pub use oci_client::Reference;
pub use storage::{ImageEntry, InsertResult, KnownPackage, StateInfo};

/// Format a byte size as a human-readable string (B, KB, MB, GB).
#[must_use]
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
