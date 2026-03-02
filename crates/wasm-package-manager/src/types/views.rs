use super::models::WitPackage;

/// A public view of a WIT package, without internal database IDs.
///
/// This type is freely constructable and is the primary public API type
/// for representing WIT packages. Internal code uses [`WitPackage`]
/// with database IDs; this view type strips those away.
#[derive(Debug, Clone)]
pub struct WitPackageView {
    /// The WIT package name (e.g. "wasi:http").
    pub package_name: String,
    /// Semver version string, if known.
    pub version: Option<String>,
    /// Human-readable description of the type.
    pub description: Option<String>,
    /// Full WIT text representation, when available.
    pub wit_text: Option<String>,
    /// When this row was created.
    pub created_at: String,
}

impl From<WitPackage> for WitPackageView {
    fn from(wt: WitPackage) -> Self {
        Self {
            package_name: wt.package_name,
            version: wt.version,
            description: wt.description,
            wit_text: wt.wit_text,
            created_at: wt.created_at,
        }
    }
}
