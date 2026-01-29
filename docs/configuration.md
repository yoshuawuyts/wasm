# Configuration

`wasm(1)` uses a local storage system to manage downloaded packages and metadata. This guide explains the storage layout and configuration options.

## Storage Location

`wasm(1)` follows the XDG Base Directory specification for storing data:

| XDG Variable       | Purpose                                                | Linux/BSD                     | macOS                             | Windows               |
| ------------------ | ------------------------------------------------------ | ----------------------------- | --------------------------------- | --------------------- |
| `$XDG_CONFIG_HOME` | User-specific configuration files                      | `~/.config`                   | `~/Library/Preferences`           | `%LOCALAPPDATA%`      |
| `$XDG_DATA_HOME`   | User-specific data files                               | `~/.local/share`              | `~/Library/Application Support`   | `%LOCALAPPDATA%`      |
| `$XDG_STATE_HOME`  | User-specific state data (logs, history, recent files) | `~/.local/state`              | `~/Library/Application Support`   | `%LOCALAPPDATA%`      |

Configuration is stored at `$XDG_CONFIG_HOME/wasm/config.json`, and data is stored at `$XDG_DATA_HOME/wasm`.

## Storage Layout

The storage directory has the following structure:

```
~/.local/share/wasm/
├── store/              # Content-addressable blob storage
│   ├── content/        # OCI image layers and artifacts
│   └── index/          # Cache index files
└── db/
    └── metadata.db3    # SQLite database with package metadata
```

### Components

#### Content-Addressable Store (`store/`)

The store directory uses [`cacache`](https://docs.rs/cacache/) for content-addressable storage. It stores the entire OCI image including any signatures and attestations:

- **Immutable**: Content is stored by its SHA-256 hash
- **Deduplicated**: Identical content is stored only once
- **OCI-Compatible**: Stores image layers and manifests following OCI specifications

#### Metadata Database (`db/metadata.db3`)

The metadata database is a SQLite database that stores package metadata.

## Configuration Options

`wasm(1)` uses a JSON configuration file for managing registry-specific settings. The configuration file is located at:

- **Linux/BSD**: `~/.config/wasm/config.json`
- **macOS**: `~/Library/Preferences/wasm/config.json`
- **Windows**: `%LOCALAPPDATA%\wasm\config.json`

If the configuration file doesn't exist, a default configuration will be created automatically on first use.

### Configuration Format

The configuration file uses JSON format for compatibility with tools like [1Password](https://1password.com/). Here's an example:

```json
{
  "registries": {
    "ghcr.io": {
      "auth_command": "op read op://vault/github/token"
    },
    "docker.io": {
      "auth_command": "op read op://vault/docker/token"
    },
    "my-registry.example.com": {
      "anonymous": true
    }
  }
}
```

### Per-Registry Configuration

Each registry can have its own configuration with the following options:

#### `auth_command` (optional)

A command to execute to retrieve authentication credentials for the registry. This is useful for integrating with secret managers like 1Password.

The command should output credentials in a format compatible with Docker credential helpers.

**Example using 1Password CLI:**

```json
{
  "registries": {
    "ghcr.io": {
      "auth_command": "op read op://vault/github/token"
    }
  }
}
```

#### `anonymous` (optional, default: `false`)

Whether to use anonymous authentication for this registry. When set to `true`, no credentials will be used even if available through Docker credential helpers.

**Example:**

```json
{
  "registries": {
    "public-registry.example.com": {
      "anonymous": true
    }
  }
}
```

### Authentication Priority

When pulling from a registry, `wasm(1)` uses the following priority for authentication:

1. **Registry-specific configuration**: If an `auth_command` is defined for the registry, it will be used
2. **Anonymous override**: If `anonymous: true` is set, no credentials will be used
3. **Docker credential helpers**: Falls back to Docker/Podman credential helpers (see [Authentication](./authentication.md))
4. **Anonymous access**: If no credentials are found, attempts anonymous access

### 1Password Integration

To integrate with 1Password, you can use the `op` CLI tool:

1. Install the [1Password CLI](https://developer.1password.com/docs/cli/get-started/)
2. Sign in to your 1Password account: `op signin`
3. Store your registry credentials in 1Password
4. Configure `wasm(1)` to use 1Password for credentials:

```json
{
  "registries": {
    "ghcr.io": {
      "auth_command": "op read op://vault/github-token/credential"
    }
  }
}
```

Replace `vault`, `github-token`, and `credential` with your actual 1Password vault, item, and field names.

## Storage Management

### Viewing Storage Usage

Check storage usage with:

```bash
wasm self state
```

### Cleaning Up Storage

Remove unused content and optimize the database:

```bash
wasm self clean
```

This command:
- Removes orphaned content from the store
- Vacuums the SQLite database
- Reclaims disk space

### Listing Stored Packages

View all locally stored packages:

```bash
wasm package list
```

## Database Migrations

The storage system uses a migration system to evolve the schema over time:

- **Automatic**: Migrations run automatically when opening the database
- **Versioned**: Each migration is numbered and tracked
- **Forward-Only**: The system only supports forward migrations

Current migrations include:
1. Initial schema creation
2. Known packages table
3. Tag type classification
