use super::models::RawKnownPackage;

/// A public view of a known package, without internal database IDs.
///
/// This type is freely constructable and is the primary public API type
/// for representing known packages. Internal code uses [`RawKnownPackage`]
/// with database IDs; this type strips those away.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KnownPackage {
    /// Registry hostname
    pub registry: String,
    /// Repository path
    pub repository: String,
    /// Optional package description
    pub description: Option<String>,
    /// Release tags
    pub tags: Vec<String>,
    /// Signature tags (kept for API compatibility, always empty)
    #[serde(default)]
    pub signature_tags: Vec<String>,
    /// Attestation tags (kept for API compatibility, always empty)
    #[serde(default)]
    pub attestation_tags: Vec<String>,
    /// Timestamp of last seen
    pub last_seen_at: String,
    /// Timestamp of creation
    pub created_at: String,
}

impl KnownPackage {
    /// Returns the full reference string for this package (e.g., "ghcr.io/user/repo").
    #[must_use]
    pub fn reference(&self) -> String {
        format!("{}/{}", self.registry, self.repository)
    }

    /// Returns the full reference string with the most recent tag.
    #[must_use]
    pub fn reference_with_tag(&self) -> String {
        if let Some(tag) = self.tags.first() {
            format!("{}:{}", self.reference(), tag)
        } else {
            format!("{}:latest", self.reference())
        }
    }
}

impl From<RawKnownPackage> for KnownPackage {
    fn from(pkg: RawKnownPackage) -> Self {
        Self {
            registry: pkg.registry,
            repository: pkg.repository,
            description: pkg.description,
            tags: pkg.tags,
            signature_tags: pkg.signature_tags,
            attestation_tags: pkg.attestation_tags,
            last_seen_at: pkg.last_seen_at,
            created_at: pkg.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── KnownPackage ────────────────────────────────────────────────────

    #[test]
    fn known_package_reference() {
        let pkg = KnownPackage {
            registry: "ghcr.io".into(),
            repository: "user/repo".into(),
            description: None,
            tags: vec![],
            signature_tags: vec![],
            attestation_tags: vec![],
            last_seen_at: String::new(),
            created_at: String::new(),
        };
        assert_eq!(pkg.reference(), "ghcr.io/user/repo");
    }

    #[test]
    fn known_package_reference_with_tag() {
        let pkg = KnownPackage {
            registry: "ghcr.io".into(),
            repository: "user/repo".into(),
            description: None,
            tags: vec!["v1.0".into(), "latest".into()],
            signature_tags: vec![],
            attestation_tags: vec![],
            last_seen_at: String::new(),
            created_at: String::new(),
        };
        assert_eq!(pkg.reference_with_tag(), "ghcr.io/user/repo:v1.0");
    }

    #[test]
    fn known_package_reference_with_tag_default() {
        let pkg = KnownPackage {
            registry: "ghcr.io".into(),
            repository: "user/repo".into(),
            description: None,
            tags: vec![],
            signature_tags: vec![],
            attestation_tags: vec![],
            last_seen_at: String::new(),
            created_at: String::new(),
        };
        assert_eq!(pkg.reference_with_tag(), "ghcr.io/user/repo:latest");
    }
}
