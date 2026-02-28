//! Pure logic extracted from the `Manager` and `Store` implementations.
//!
//! These functions contain no IO and can be unit-tested in isolation.

use oci_client::manifest::OciDescriptor;

/// Truncated digest length used in vendor filenames.
const DIGEST_PREFIX_LEN: usize = 12;

/// Compute the vendor filename for a cached layer.
///
/// The filename encodes the registry, repository, optional tag, and a
/// truncated image digest so that vendored files are human-identifiable.
#[must_use]
pub fn vendor_filename(
    registry: &str,
    repository: &str,
    tag: Option<&str>,
    digest: &str,
) -> String {
    let registry_part = registry.replace('.', "-");
    let repo_part = repository.replace('/', "-");
    let tag_part = tag.map(|t| format!("-{t}")).unwrap_or_default();
    let sha_part = digest.strip_prefix("sha256:").unwrap_or(digest);
    let short_sha = sha_part.get(..DIGEST_PREFIX_LEN).unwrap_or(sha_part);
    format!("{registry_part}-{repo_part}{tag_part}-{short_sha}.wasm")
}

/// Determine whether a sync from the meta-registry should proceed.
///
/// Returns `true` when enough time has elapsed since `last_synced_epoch`,
/// or when the last-sync timestamp is unknown.
#[must_use]
pub fn should_sync(last_synced_epoch: Option<i64>, sync_interval: u64, now_epoch: i64) -> bool {
    match last_synced_epoch {
        Some(last) => now_epoch - last >= sync_interval as i64,
        None => true,
    }
}

/// Filter manifest layers to only those with `application/wasm` media type.
#[must_use]
pub fn filter_wasm_layers(layers: &[OciDescriptor]) -> Vec<&OciDescriptor> {
    layers
        .iter()
        .filter(|l| l.media_type == "application/wasm")
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── vendor_filename ─────────────────────────────────────────────────

    #[test]
    fn vendor_filename_basic() {
        let name = vendor_filename(
            "ghcr.io",
            "user/repo",
            Some("v1.0"),
            "sha256:abcdef1234567890",
        );
        assert_eq!(name, "ghcr-io-user-repo-v1.0-abcdef123456.wasm");
    }

    #[test]
    fn vendor_filename_no_tag() {
        let name = vendor_filename("ghcr.io", "user/repo", None, "sha256:abcdef1234567890");
        assert_eq!(name, "ghcr-io-user-repo-abcdef123456.wasm");
    }

    #[test]
    fn vendor_filename_short_digest() {
        let name = vendor_filename("ghcr.io", "user/repo", Some("latest"), "sha256:abc");
        assert_eq!(name, "ghcr-io-user-repo-latest-abc.wasm");
    }

    #[test]
    fn vendor_filename_no_sha256_prefix() {
        let name = vendor_filename("docker.io", "lib/hello", Some("1.0"), "rawdigest123456");
        assert_eq!(name, "docker-io-lib-hello-1.0-rawdigest123.wasm");
    }

    #[test]
    fn vendor_filename_nested_repository() {
        let name = vendor_filename(
            "ghcr.io",
            "org/team/component",
            Some("v2"),
            "sha256:0123456789abcdef",
        );
        assert_eq!(name, "ghcr-io-org-team-component-v2-0123456789ab.wasm");
    }

    #[test]
    fn vendor_filename_unknown_digest() {
        let name = vendor_filename("ghcr.io", "user/repo", None, "unknown");
        assert_eq!(name, "ghcr-io-user-repo-unknown.wasm");
    }

    // ── should_sync ─────────────────────────────────────────────────────

    #[test]
    fn should_sync_no_previous() {
        assert!(should_sync(None, 3600, 1000));
    }

    #[test]
    fn should_sync_stale() {
        assert!(should_sync(Some(1000), 3600, 5000));
    }

    #[test]
    fn should_sync_fresh() {
        assert!(!should_sync(Some(1000), 3600, 2000));
    }

    #[test]
    fn should_sync_exact_boundary() {
        // Exactly at the interval boundary should trigger sync.
        assert!(should_sync(Some(0), 3600, 3600));
    }

    // ── filter_wasm_layers ──────────────────────────────────────────────

    #[test]
    fn filter_wasm_layers_mixed() {
        let layers = vec![
            OciDescriptor {
                media_type: "application/wasm".to_string(),
                digest: "sha256:aaa".to_string(),
                size: 100,
                urls: None,
                annotations: None,
            },
            OciDescriptor {
                media_type: "application/vnd.oci.image.config.v1+json".to_string(),
                digest: "sha256:bbb".to_string(),
                size: 50,
                urls: None,
                annotations: None,
            },
            OciDescriptor {
                media_type: "application/wasm".to_string(),
                digest: "sha256:ccc".to_string(),
                size: 200,
                urls: None,
                annotations: None,
            },
        ];
        let wasm = filter_wasm_layers(&layers);
        assert_eq!(wasm.len(), 2);
        assert_eq!(wasm[0].digest, "sha256:aaa");
        assert_eq!(wasm[1].digest, "sha256:ccc");
    }

    #[test]
    fn filter_wasm_layers_none() {
        let layers = vec![OciDescriptor {
            media_type: "application/json".to_string(),
            digest: "sha256:xxx".to_string(),
            size: 10,
            urls: None,
            annotations: None,
        }];
        assert!(filter_wasm_layers(&layers).is_empty());
    }

    #[test]
    fn filter_wasm_layers_empty() {
        assert!(filter_wasm_layers(&[]).is_empty());
    }
}
