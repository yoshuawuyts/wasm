use oci_client::Reference;

use crate::network::Client;
use crate::storage::{ImageEntry, InsertResult, KnownPackage, StateInfo, Store, WitInterface};

/// A cache on disk
#[derive(Debug)]
pub struct Manager {
    client: Client,
    store: Store,
}

impl Manager {
    /// Create a new store at a location on disk.
    ///
    /// This may return an error if it fails to create the cache location on disk.
    pub async fn open() -> anyhow::Result<Self> {
        let client = Client::new();
        let store = Store::open().await?;

        Ok(Self { client, store })
    }

    // /// Create a new store at a location on disk.
    // ///
    // /// This may return an error if it fails to create the cache location on disk.
    // pub async fn with_config(config: Config) -> anyhow::Result<Self> {
    //     let client = Client::new();
    //     let store = Store::open().await?;

    //     Ok(Self { client, store })
    // }

    /// Pull a package from the registry.
    /// Returns the insert result indicating whether the package was newly inserted
    /// or already existed in the database.
    ///
    /// This method also fetches all related tags for the package and stores them
    /// as known packages for discovery purposes.
    pub async fn pull(&self, reference: Reference) -> anyhow::Result<InsertResult> {
        let image = self.client.pull(&reference).await?;
        let result = self.store.insert(&reference, image).await?;

        // Add to known packages when pulling (with tag if present)
        self.store.add_known_package(
            reference.registry(),
            reference.repository(),
            reference.tag(),
            None,
        )?;

        // Fetch all related tags and store them as known packages
        if let Ok(tags) = self.client.list_tags(&reference).await {
            for tag in tags {
                self.store.add_known_package(
                    reference.registry(),
                    reference.repository(),
                    Some(&tag),
                    None,
                )?;
            }
        }

        Ok(result)
    }

    /// List all stored images and their metadata.
    pub fn list_all(&self) -> anyhow::Result<Vec<ImageEntry>> {
        self.store.list_all()
    }

    /// Get data from the store
    pub async fn get(&self, key: &str) -> cacache::Result<Vec<u8>> {
        cacache::read(self.store.state_info.layers_dir(), key).await
    }

    /// Get information about the current state of the package manager.
    pub fn state_info(&self) -> StateInfo {
        self.store.state_info.clone()
    }

    /// Delete an image from the store by its reference.
    pub async fn delete(&self, reference: Reference) -> anyhow::Result<bool> {
        self.store.delete(&reference).await
    }

    /// Search for known packages by query string.
    /// Searches in both registry and repository fields.
    pub fn search_packages(&self, query: &str) -> anyhow::Result<Vec<KnownPackage>> {
        self.store.search_known_packages(query)
    }

    /// Get all known packages.
    pub fn list_known_packages(&self) -> anyhow::Result<Vec<KnownPackage>> {
        self.store.list_known_packages()
    }

    /// Add or update a known package entry.
    pub fn add_known_package(
        &self,
        registry: &str,
        repository: &str,
        tag: Option<&str>,
        description: Option<&str>,
    ) -> anyhow::Result<()> {
        self.store
            .add_known_package(registry, repository, tag, description)
    }

    /// List all tags for a given reference from the registry.
    pub async fn list_tags(&self, reference: &Reference) -> anyhow::Result<Vec<String>> {
        self.client.list_tags(reference).await
    }
    
    /// List all WIT interfaces with their associated component references.
    pub fn list_wit_interfaces(&self) -> anyhow::Result<Vec<(WitInterface, String)>> {
        self.store.list_wit_interfaces_with_components()
    }
}
