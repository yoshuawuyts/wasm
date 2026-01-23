use docker_credential::DockerCredential;
use oci_client::Reference;
use oci_client::client::{ClientConfig, ClientProtocol, ImageData};
use oci_client::secrets::RegistryAuth;
use oci_wasm::WasmClient;

pub(crate) struct Client {
    inner: WasmClient,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client").finish_non_exhaustive()
    }
}

impl Client {
    pub(crate) fn new() -> Self {
        let config = ClientConfig {
            protocol: ClientProtocol::Https,
            ..Default::default()
        };
        let client = WasmClient::new(oci_client::Client::new(config));
        Self { inner: client }
    }

    pub async fn pull(&self, reference: &Reference) -> anyhow::Result<ImageData> {
        let auth = resolve_auth(&reference)?;
        let image = self.inner.pull(&reference, &auth).await?;
        Ok(image)
    }
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
