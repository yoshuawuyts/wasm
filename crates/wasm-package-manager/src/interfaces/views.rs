use super::models::WitInterface;

/// A public view of a WIT interface, without internal database IDs.
///
/// This type is freely constructable and is the primary public API type
/// for representing WIT interfaces. Internal code uses [`WitInterface`]
/// with database IDs; this view type strips those away.
#[derive(Debug, Clone)]
pub struct WitInterfaceView {
    /// The WIT package name (e.g. "wasi:http").
    pub package_name: String,
    /// Semver version string, if known.
    pub version: Option<String>,
    /// Human-readable description of the interface.
    pub description: Option<String>,
    /// Full WIT text representation, when available.
    pub wit_text: Option<String>,
    /// When this row was created.
    pub created_at: String,
}

impl From<WitInterface> for WitInterfaceView {
    fn from(iface: WitInterface) -> Self {
        Self {
            package_name: iface.package_name,
            version: iface.version,
            description: iface.description,
            wit_text: iface.wit_text,
            created_at: iface.created_at,
        }
    }
}
