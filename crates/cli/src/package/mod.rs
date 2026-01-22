use anyhow::Result;
use oci_client::Reference;

/// Package, push, and pull Wasm Components
#[derive(clap::Parser)]
pub(crate) enum Opts {
    /// Fetch OCI metadata for a component
    Show,
    /// Pull a component from the registry
    Pull(PullOpts),
    Push,
}

#[derive(clap::Args)]
pub(crate) struct PullOpts {
    /// The reference to pull
    reference: Reference,
}

impl Opts {
    pub(crate) async fn run(self) -> Result<()> {
        match self {
            Opts::Show => todo!(),
            Opts::Pull(opts) => pull::pull(opts).await,
            Opts::Push => todo!(),
        }
    }
}

mod pull {
    use anyhow::Context;
    use docker_credential::DockerCredential;
    use oci_client::Reference;
    use oci_client::client::{ClientConfig, ClientProtocol};
    use oci_client::secrets::RegistryAuth;
    use oci_wasm::WasmClient;

    pub(crate) async fn pull(opts: super::PullOpts) -> anyhow::Result<()> {
        let config = ClientConfig {
            protocol: ClientProtocol::Https,
            ..Default::default()
        };
        let client = WasmClient::new(oci_client::Client::new(config));

        let auth = resolve_auth(&opts.reference)?;
        let image = client
            .pull(&opts.reference, &auth)
            .await
            .context("Unable to pull image")?;
        dbg!(image.digest);
        Ok(())
    }

    fn resolve_auth(reference: &Reference) -> anyhow::Result<RegistryAuth> {
        // NOTE: copied approach from https://github.com/bytecodealliance/wasm-pkg-tools/blob/48c28825a7dfb585b3fe1d42be65fe73a17d84fe/crates/wkg/src/oci.rs#L59-L66
        let server_url = match reference.resolve_registry() {
            "index.docker.io" => "https://index.docker.io/v1/", // Default registry uses this key.
            other => other, // All other registries are keyed by their domain name without the `https://` prefix or any path suffix.
        };

        match docker_credential::get_credential(server_url) {
            Ok(DockerCredential::UsernamePassword(username, password)) => {
                return Ok(RegistryAuth::Basic(username, password));
            }
            Ok(DockerCredential::IdentityToken(_)) => {
                return Err(anyhow::anyhow!("identity tokens not supported"));
            }
            Err(_) => Ok(RegistryAuth::Anonymous),
        }
    }
}
