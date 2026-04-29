//! `wasm registry notify` subcommand.

#![allow(clippy::print_stdout, clippy::print_stderr)]

use anyhow::{Result, bail};
use wasm_meta_registry_types::NotifyOutcome;
use wasm_package_manager::Reference;
use wasm_package_manager::manager::{Manager, SyncPolicy, install};

/// Default meta-registry URL.
const DEFAULT_REGISTRY_URL: &str = Manager::DEFAULT_REGISTRY_URL;

/// Notify a meta-registry that a new version of a package is available.
///
/// Sends a hint to the meta-registry asking it to fetch the given tag as
/// soon as possible, instead of waiting for the next periodic sync.
#[derive(clap::Args)]
pub(crate) struct NotifyOpts {
    /// The newly-published package, given as either a WIT-style name
    /// (e.g., `wasi:http@0.2.11`) or a full OCI reference
    /// (e.g., `ghcr.io/example/component:1.2.3`).
    package: String,

    /// URL of the meta-registry to notify.
    #[arg(long, default_value = DEFAULT_REGISTRY_URL)]
    registry_url: String,
}

impl NotifyOpts {
    pub(crate) async fn run(self, offline: bool) -> Result<()> {
        // The notify endpoint requires an HTTP request to the meta-registry,
        // so refuse early in offline mode before opening the store.
        if offline {
            bail!("cannot notify meta-registry in offline mode");
        }

        let manager = Manager::open().await?;
        let reference = resolve_reference(&self.package, &manager).await?;

        let registry = reference.registry();
        let repository = reference.repository();
        let Some(tag) = reference.tag() else {
            bail!(
                "'{}' has no version; specify one (e.g., `wasi:http@0.2.11` or `ghcr.io/example/component:1.2.3`) so the registry knows which version to fetch",
                self.package
            );
        };

        let outcome = manager
            .notify_meta_registry(&self.registry_url, registry, repository, tag)
            .await?;

        match outcome {
            NotifyOutcome::Enqueued => {
                println!(
                    "Notified {}: '{}' enqueued for fetch",
                    self.registry_url,
                    reference.whole()
                );
            }
            NotifyOutcome::Skipped { reason } => {
                println!(
                    "Notified {}: '{}' skipped ({reason})",
                    self.registry_url,
                    reference.whole()
                );
            }
        }
        Ok(())
    }
}

/// Resolve user input to an OCI [`Reference`].
///
/// Accepts either a WIT-style name (`namespace:package@version`), which is
/// looked up in the known-package index, or a full OCI reference.
async fn resolve_reference(input: &str, manager: &Manager) -> Result<Reference> {
    if install::looks_like_wit_name(input) {
        // Refresh the known-package index so resolution can find packages
        // that haven't been touched locally yet. Failures here are
        // non-fatal — fall through to the local lookup.
        let _ = manager
            .sync_from_meta_registry(
                Manager::DEFAULT_REGISTRY_URL,
                Manager::DEFAULT_SYNC_INTERVAL,
                SyncPolicy::IfStale,
            )
            .await;
        return install::resolve_wit_name(input, manager);
    }

    wasm_package_manager::parse_reference(input)
        .map_err(|e| anyhow::anyhow!("invalid package reference '{input}': {e}"))
}
