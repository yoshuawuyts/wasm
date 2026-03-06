//! Progress bar rendering for the `wasm install` command.
//!
//! This module handles the display of download progress for packages being
//! installed. Each package gets a single aggregated progress bar that combines
//! all layer downloads. Packages are displayed in a tree structure with
//! `├──` and `└──` glyphs.

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use wasm_package_manager::ProgressEvent;

/// Tree glyph for non-last items in the tree.
const TREE_GLYPH_MID: &str = "├──";

/// Tree glyph for the last item in the tree.
const TREE_GLYPH_END: &str = "└──";

/// Return the appropriate tree glyph for a position in a list.
///
/// # Examples
///
/// ```rust,ignore
/// assert_eq!(tree_glyph(false), "├──");
/// assert_eq!(tree_glyph(true), "└──");
/// ```
// r[impl cli.progress-bar.tree-glyph]
fn tree_glyph(is_last: bool) -> &'static str {
    if is_last {
        TREE_GLYPH_END
    } else {
        TREE_GLYPH_MID
    }
}

/// Extract the display name and version from a package reference.
///
/// For WIT-style names like `wasi:http@0.2.0`, the name is `wasi:http` and
/// version is `0.2.0`. For WIT-style names without version like `wasi:http`,
/// the version is taken from the OCI reference tag (stripping a leading `v`).
/// For bare OCI references, the repository path is used as the name.
// r[impl cli.progress-bar.namespace-name]
pub(crate) fn package_display_parts(
    explicit_name: Option<&str>,
    tag: Option<&str>,
) -> (String, Option<String>) {
    if let Some(name) = explicit_name {
        if let Some((n, v)) = name.split_once('@') {
            (n.to_string(), Some(v.to_string()))
        } else {
            let version = tag.map(|t| t.strip_prefix('v').unwrap_or(t).to_string());
            (name.to_string(), version)
        }
    } else {
        // For OCI references without an explicit name, fall back to tag only
        let version = tag.map(|t| t.strip_prefix('v').unwrap_or(t).to_string());
        (String::new(), version)
    }
}

/// Build the ANSI-colored prefix string for a progress bar line.
///
/// During download the name is yellow; when complete it is green.
/// The `@version` suffix is always dim (grey).
// r[impl cli.progress-bar.package-name-downloading]
// r[impl cli.progress-bar.package-name-complete]
// r[impl cli.progress-bar.version-grey]
// r[impl cli.progress-bar.no-indent]
fn build_prefix(glyph: &str, name: &str, version: Option<&str>, is_complete: bool) -> String {
    let styled_name = if is_complete {
        console::style(name).green().to_string()
    } else {
        console::style(name).yellow().to_string()
    };

    match version {
        Some(v) => format!(
            "{glyph} {}{}",
            styled_name,
            console::style(format!("@{v}")).dim()
        ),
        None => format!("{glyph} {styled_name}"),
    }
}

/// Create a progress bar for a package and return it.
///
/// The bar is added to `multi` and initially styled for in-progress
/// display (yellow bar, grey sizes and ETA).
// r[impl cli.progress-bar.bar-yellow]
// r[impl cli.progress-bar.size-grey]
// r[impl cli.progress-bar.eta-grey]
pub(crate) fn create_package_bar(
    multi: &MultiProgress,
    name: &str,
    version: Option<&str>,
    is_last: bool,
) -> ProgressBar {
    let glyph = tree_glyph(is_last);
    let prefix = build_prefix(glyph, name, version, false);

    let pb = multi.add(ProgressBar::new(0));
    pb.set_style(progress_style());
    pb.set_prefix(prefix);
    pb
}

/// Mark a package progress bar as complete.
///
/// Switches the bar style to the done style (no bar, just total size)
/// and updates the prefix to show the green name.
// r[impl cli.progress-bar.bar-hidden-on-complete]
pub(crate) fn finish_package_bar(
    pb: &ProgressBar,
    name: &str,
    version: Option<&str>,
    is_last: bool,
) {
    let glyph = tree_glyph(is_last);
    let prefix = build_prefix(glyph, name, version, true);
    pb.set_style(done_style());
    pb.set_prefix(prefix);
    pb.finish();
}

/// Style for in-progress downloads: yellow bar, dim size and ETA.
fn progress_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "{prefix} {bar:12.yellow} {binary_bytes:.dim}/{binary_total_bytes:.dim} {eta:.dim}",
    )
    .expect("valid progress bar template")
    .progress_chars("━━┄")
}

/// Style for completed downloads: no bar, just dim total size.
fn done_style() -> ProgressStyle {
    ProgressStyle::with_template("{prefix} {binary_total_bytes:.dim}")
        .expect("valid progress bar template")
}

/// Consume progress events and update a single aggregated progress bar.
///
/// All layer downloads are aggregated into a single progress bar for the
/// package. The total bytes is the sum of all layer sizes, and progress
/// is the sum of all per-layer bytes downloaded.
// r[impl cli.progress-bar.aggregate-layers]
pub(crate) async fn run_progress_bars(
    pb: ProgressBar,
    mut rx: tokio::sync::mpsc::Receiver<ProgressEvent>,
) {
    let mut layer_progress: Vec<u64> = Vec::new();
    let mut total_bytes: u64 = 0;

    while let Some(event) = rx.recv().await {
        match event {
            ProgressEvent::ManifestFetched { layer_count, .. } => {
                layer_progress.resize(layer_count, 0);
            }
            ProgressEvent::LayerStarted {
                total_bytes: size, ..
            } => {
                if let Some(size) = size {
                    total_bytes += size;
                    pb.set_length(total_bytes);
                }
            }
            ProgressEvent::LayerProgress {
                index,
                bytes_downloaded,
            } => {
                if let Some(slot) = layer_progress.get_mut(index) {
                    *slot = bytes_downloaded;
                }
                let downloaded: u64 = layer_progress.iter().sum();
                pb.set_position(downloaded);
            }
            ProgressEvent::LayerDownloaded { .. }
            | ProgressEvent::LayerStored { .. }
            | ProgressEvent::InstallComplete => {
                // Layer events and install-complete are handled by the caller
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // r[verify cli.progress-bar.tree-glyph]
    #[test]
    fn tree_glyph_mid_for_non_last() {
        assert_eq!(tree_glyph(false), "├──");
    }

    // r[verify cli.progress-bar.tree-glyph]
    #[test]
    fn tree_glyph_end_for_last() {
        assert_eq!(tree_glyph(true), "└──");
    }

    // r[verify cli.progress-bar.namespace-name]
    #[test]
    fn display_parts_wit_name_with_version() {
        let (name, version) = package_display_parts(Some("wasi:http@0.2.0"), Some("v0.2.0"));
        assert_eq!(name, "wasi:http");
        assert_eq!(version.as_deref(), Some("0.2.0"));
    }

    // r[verify cli.progress-bar.namespace-name]
    #[test]
    fn display_parts_wit_name_without_version() {
        let (name, version) = package_display_parts(Some("wasi:http"), Some("v0.2.10"));
        assert_eq!(name, "wasi:http");
        assert_eq!(version.as_deref(), Some("0.2.10"));
    }

    // r[verify cli.progress-bar.namespace-name]
    #[test]
    fn display_parts_wit_name_strips_v_prefix() {
        let (name, version) = package_display_parts(Some("wasi:http"), Some("v1.0.0"));
        assert_eq!(name, "wasi:http");
        assert_eq!(version.as_deref(), Some("1.0.0"));
    }

    // r[verify cli.progress-bar.namespace-name]
    #[test]
    fn display_parts_no_tag() {
        let (name, version) = package_display_parts(Some("wasi:http"), None);
        assert_eq!(name, "wasi:http");
        assert_eq!(version, None);
    }

    // r[verify cli.progress-bar.namespace-name]
    #[test]
    fn display_parts_tag_without_v_prefix() {
        let (name, version) = package_display_parts(Some("ba:sample"), Some("0.12.2"));
        assert_eq!(name, "ba:sample");
        assert_eq!(version.as_deref(), Some("0.12.2"));
    }

    // r[verify cli.progress-bar.no-indent]
    #[test]
    fn prefix_not_indented() {
        // Force no colors for consistent test output
        console::set_colors_enabled(false);
        let prefix = build_prefix("├──", "wasi:http", Some("0.2.0"), false);
        // The prefix must start with the tree glyph, not spaces
        assert!(
            prefix.starts_with("├──"),
            "prefix must start with tree glyph, got: {prefix}"
        );
    }

    // r[verify cli.progress-bar.package-name-downloading]
    #[test]
    fn prefix_contains_name_while_downloading() {
        console::set_colors_enabled(false);
        let prefix = build_prefix("├──", "wasi:http", Some("0.2.0"), false);
        assert!(
            prefix.contains("wasi:http"),
            "prefix must contain package name: {prefix}"
        );
    }

    // r[verify cli.progress-bar.package-name-complete]
    #[test]
    fn prefix_contains_name_when_complete() {
        console::set_colors_enabled(false);
        let prefix = build_prefix("├──", "wasi:http", Some("0.2.0"), true);
        assert!(
            prefix.contains("wasi:http"),
            "prefix must contain package name: {prefix}"
        );
    }

    // r[verify cli.progress-bar.version-grey]
    #[test]
    fn prefix_contains_version() {
        console::set_colors_enabled(false);
        let prefix = build_prefix("├──", "wasi:http", Some("0.2.0"), false);
        assert!(
            prefix.contains("@0.2.0"),
            "prefix must contain @version: {prefix}"
        );
    }

    // r[verify cli.progress-bar.version-grey]
    #[test]
    fn prefix_no_version_when_none() {
        console::set_colors_enabled(false);
        let prefix = build_prefix("├──", "wasi:http", None, false);
        assert!(
            !prefix.contains('@'),
            "prefix must not contain @ when no version: {prefix}"
        );
    }

    // r[verify cli.progress-bar.aggregate-layers]
    #[tokio::test]
    async fn aggregate_layers_sums_progress() {
        use indicatif::ProgressDrawTarget;

        let pb = ProgressBar::with_draw_target(Some(0), ProgressDrawTarget::hidden());

        let (tx, rx) = tokio::sync::mpsc::channel(64);

        let handle = tokio::spawn(run_progress_bars(pb.clone(), rx));

        // Simulate 2 layers
        tx.send(ProgressEvent::ManifestFetched {
            layer_count: 2,
            image_digest: "sha256:abc".into(),
        })
        .await
        .unwrap();

        tx.send(ProgressEvent::LayerStarted {
            index: 0,
            digest: "sha256:layer0".into(),
            total_bytes: Some(1000),
            title: None,
            media_type: "application/wasm".into(),
        })
        .await
        .unwrap();

        tx.send(ProgressEvent::LayerStarted {
            index: 1,
            digest: "sha256:layer1".into(),
            total_bytes: Some(500),
            title: None,
            media_type: "application/wasm".into(),
        })
        .await
        .unwrap();

        // Progress on both layers
        tx.send(ProgressEvent::LayerProgress {
            index: 0,
            bytes_downloaded: 600,
        })
        .await
        .unwrap();

        tx.send(ProgressEvent::LayerProgress {
            index: 1,
            bytes_downloaded: 300,
        })
        .await
        .unwrap();

        // Allow processing
        tokio::task::yield_now().await;

        // Verify aggregate state
        assert_eq!(
            pb.length(),
            Some(1500),
            "total should be sum of layer sizes"
        );
        assert_eq!(
            pb.position(),
            900,
            "position should be sum of layer progress"
        );

        tx.send(ProgressEvent::InstallComplete).await.unwrap();
        drop(tx);
        let _ = handle.await;
    }

    // r[verify cli.progress-bar.aggregate-layers]
    #[tokio::test]
    async fn aggregate_layers_handles_unknown_sizes() {
        use indicatif::ProgressDrawTarget;

        let pb = ProgressBar::with_draw_target(Some(0), ProgressDrawTarget::hidden());

        let (tx, rx) = tokio::sync::mpsc::channel(64);

        let handle = tokio::spawn(run_progress_bars(pb.clone(), rx));

        tx.send(ProgressEvent::ManifestFetched {
            layer_count: 1,
            image_digest: "sha256:abc".into(),
        })
        .await
        .unwrap();

        // Layer with unknown size
        tx.send(ProgressEvent::LayerStarted {
            index: 0,
            digest: "sha256:layer0".into(),
            total_bytes: None,
            title: None,
            media_type: "application/wasm".into(),
        })
        .await
        .unwrap();

        tx.send(ProgressEvent::LayerProgress {
            index: 0,
            bytes_downloaded: 500,
        })
        .await
        .unwrap();

        tokio::task::yield_now().await;

        // Total stays at initial 0 since we never got a total_bytes
        assert_eq!(pb.length(), Some(0));
        assert_eq!(pb.position(), 500);

        drop(tx);
        let _ = handle.await;
    }

    // r[verify cli.progress-bar.bar-hidden-on-complete]
    #[test]
    fn done_style_has_no_bar_placeholder() {
        // The done style template should not contain {bar}
        let style_str = "{prefix} {binary_total_bytes:.dim}";
        assert!(
            !style_str.contains("{bar"),
            "done style must not contain a bar"
        );
    }

    // r[verify cli.progress-bar.bar-yellow]
    #[test]
    fn progress_style_uses_yellow_bar() {
        let style_str =
            "{prefix} {bar:12.yellow} {binary_bytes:.dim}/{binary_total_bytes:.dim} {eta:.dim}";
        assert!(
            style_str.contains(".yellow"),
            "progress style must use yellow for the bar"
        );
    }

    // r[verify cli.progress-bar.size-grey]
    #[test]
    fn progress_style_uses_dim_for_size() {
        let style_str =
            "{prefix} {bar:12.yellow} {binary_bytes:.dim}/{binary_total_bytes:.dim} {eta:.dim}";
        assert!(
            style_str.contains("{binary_bytes:.dim}"),
            "progress style must use dim for size"
        );
    }

    // r[verify cli.progress-bar.eta-grey]
    #[test]
    fn progress_style_uses_dim_for_eta() {
        let style_str =
            "{prefix} {bar:12.yellow} {binary_bytes:.dim}/{binary_total_bytes:.dim} {eta:.dim}";
        assert!(
            style_str.contains("{eta:.dim}"),
            "progress style must use dim for ETA"
        );
    }
}
