# Authentication

`wasm(1)` uses OCI-compatible authentication to access container registries for pulling and pushing WebAssembly packages.

## Overview

`wasm(1)` integrates with your system's Docker credential store to authenticate with container registries. This means if you've already authenticated with a registry using Docker or Podman, `wasm(1)` will automatically use those credentials.

## Authentication Methods

### Docker Credential Store

`wasm(1)` uses the [`docker_credential`](https://docs.rs/docker_credential/) crate to access credentials stored by Docker/Podman credential helpers.

The authentication flow:

1. When pulling or pushing a package, `wasm(1)` extracts the registry hostname from the reference
2. It queries the Docker credential store for credentials associated with that registry
3. If credentials are found, they're used for authentication
4. If no credentials are found, anonymous access is attempted

### Supported Credential Types

- **Username/Password**: Basic authentication with username and password
- **Anonymous**: No authentication (for public registries)

**Note**: Identity tokens are currently not supported.

## Setting Up Authentication

### Using Docker Login

The easiest way to set up authentication is to use Docker's login command:

```bash
# For Docker Hub
docker login

# For GitHub Container Registry
docker login ghcr.io

# For a custom registry
docker login myregistry.example.com
```

Once logged in, `wasm` will automatically use these credentials.

### Using Podman Login

If you use Podman instead of Docker:

```bash
# For GitHub Container Registry
podman login ghcr.io

# For a custom registry
podman login myregistry.example.com
```

## Troubleshooting

### Anonymous Access

If you see an "anonymous access" message, it means:
- No credentials were found for the registry
- The tool is attempting to access the registry without authentication
- This works for public repositories but will fail for private ones

### Unsupported Identity Tokens

If you receive an "identity tokens not supported" error:
- The credential store returned an identity token
- `wasm` currently only supports username/password authentication
- Try logging in again with username/password credentials

### Credential Store Not Found

If credential lookups fail:
- Ensure Docker or Podman is installed and configured
- Verify you've logged in to the registry at least once
- Check that credential helpers are properly configured in `~/.docker/config.json`

## Future Enhancements

Planned authentication improvement: [Support for identity tokens](https://github.com/yoshuawuyts/wasm/issues/56)
