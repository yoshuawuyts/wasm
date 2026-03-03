use oci_client::manifest::OciImageManifest;

use crate::oci::InsertResult;

/// Result of syncing the package index from a meta-registry.
#[derive(Debug)]
pub enum SyncResult {
    /// Sync was skipped because the minimum interval has not elapsed.
    Skipped,
    /// The server indicated the local data is still current (304 Not Modified).
    NotModified,
    /// New package data was fetched and stored locally.
    Updated {
        /// Number of packages that were synced.
        count: usize,
    },
    /// The sync failed but local cached data is available.
    Degraded {
        /// A human-readable description of the error.
        error: String,
    },
}

/// Controls whether `sync_from_meta_registry` respects the minimum sync
/// interval or forces an immediate fetch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncPolicy {
    /// Only sync if the minimum interval has elapsed since the last sync.
    IfStale,
    /// Ignore the minimum interval and always contact the registry.
    Force,
}

/// Result of a pull operation.
///
/// Contains the insert result along with the content digest and manifest
/// from the pulled image.
#[derive(Debug, Clone)]
pub struct PullResult {
    /// Whether the image was newly inserted or already existed.
    pub insert_result: InsertResult,
    /// The content digest of the pulled image (e.g., "sha256:abc123...").
    pub digest: Option<String>,
    /// The OCI image manifest.
    pub manifest: Option<OciImageManifest>,
}

/// Result of an install operation.
///
/// Contains metadata about the installed package for updating
/// manifest and lockfile entries.
#[derive(Debug, Clone)]
pub struct InstallResult {
    /// The registry hostname (e.g., "ghcr.io").
    pub registry: String,
    /// The repository path (e.g., "webassembly/wasi-logging").
    pub repository: String,
    /// The tag, if present (e.g., "1.0.0").
    pub tag: Option<String>,
    /// The content digest of the image.
    pub digest: Option<String>,
    /// The WIT package name if available (e.g., "wasi:logging@0.1.0").
    pub package_name: Option<String>,
    /// The `org.opencontainers.image.title` manifest annotation, if present.
    pub oci_title: Option<String>,
    /// The list of vendored file paths.
    pub vendored_files: Vec<std::path::PathBuf>,
    /// Whether this package is a compiled component (`true`) or a WIT interface (`false`).
    pub is_component: bool,
    /// Dependencies on other WIT packages extracted from the component metadata.
    pub dependencies: Vec<crate::types::DependencyItem>,
}
