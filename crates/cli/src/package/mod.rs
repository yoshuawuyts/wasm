use anyhow::Result;
use wasm_package_manager::{InsertResult, Manager, Reference};

/// Package, push, and pull Wasm Components
#[derive(clap::Parser)]
pub(crate) enum Opts {
    /// Fetch OCI metadata for a component
    Show,
    /// Pull a component from the registry
    Pull(PullOpts),
    Push,
    /// List all available tags for a component
    Tags(TagsOpts),
}

#[derive(clap::Args)]
pub(crate) struct PullOpts {
    /// The reference to pull
    reference: Reference,
}

#[derive(clap::Args)]
pub(crate) struct TagsOpts {
    /// The reference to list tags for (e.g., ghcr.io/example/component)
    reference: Reference,
    /// Include signature tags (ending in .sig)
    #[arg(long)]
    signatures: bool,
    /// Include attestation tags (ending in .att)
    #[arg(long)]
    attestations: bool,
}

impl Opts {
    pub(crate) async fn run(self) -> Result<()> {
        let store = Manager::open().await?;
        match self {
            Opts::Show => todo!(),
            Opts::Pull(opts) => {
                let result = store.pull(opts.reference.clone()).await?;
                if result == InsertResult::AlreadyExists {
                    eprintln!(
                        "warning: package '{}' already exists in the local store",
                        opts.reference.whole()
                    );
                }
                Ok(())
            }
            Opts::Push => todo!(),
            Opts::Tags(opts) => {
                // Try to fetch tags from the network first
                let network_result = store.list_tags(&opts.reference).await;

                let (tags, from_cache) = match network_result {
                    Ok(all_tags) => {
                        // Filter tags based on flags
                        let filtered: Vec<_> = all_tags
                            .into_iter()
                            .filter(|tag| {
                                let is_sig = tag.ends_with(".sig");
                                let is_att = tag.ends_with(".att");

                                if is_sig {
                                    opts.signatures
                                } else if is_att {
                                    opts.attestations
                                } else {
                                    true // Always include release tags
                                }
                            })
                            .collect();
                        (filtered, false)
                    }
                    Err(_) => {
                        // Network failed, try cached tags
                        match store.get_cached_tags(&opts.reference)? {
                            Some((release_tags, signature_tags, attestation_tags)) => {
                                let mut tags = release_tags;
                                if opts.signatures {
                                    tags.extend(signature_tags);
                                }
                                if opts.attestations {
                                    tags.extend(attestation_tags);
                                }
                                (tags, true)
                            }
                            None => {
                                anyhow::bail!(
                                    "Failed to fetch tags for '{}' and no cached tags available",
                                    opts.reference.whole()
                                );
                            }
                        }
                    }
                };

                if tags.is_empty() {
                    println!("No tags found for '{}'", opts.reference.whole());
                } else {
                    if from_cache {
                        println!("Tags for '{}' (cached):", opts.reference.whole());
                    } else {
                        println!("Tags for '{}':", opts.reference.whole());
                    }
                    for tag in tags {
                        println!("  {}", tag);
                    }
                }
                Ok(())
            }
        }
    }
}
