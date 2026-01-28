# Configuration

`wasm(1)` uses a local storage system to manage downloaded packages and metadata. This guide explains the storage layout and configuration options.

## Storage Location

`wasm(1)` follows the XDG Base Directory specification for storing data:

| XDG Variable       | Purpose                                                | Linux/BSD                     | macOS                             | Windows               |
| ------------------ | ------------------------------------------------------ | ----------------------------- | --------------------------------- | --------------------- |
| `$XDG_CONFIG_HOME` | User-specific configuration files                      | `~/.config`                   | `~/Library/Preferences`           | `%LOCALAPPDATA%`      |
| `$XDG_DATA_HOME`   | User-specific data files                               | `~/.local/share`              | `~/Library/Application Support`   | `%LOCALAPPDATA%`      |
| `$XDG_STATE_HOME`  | User-specific state data (logs, history, recent files) | `~/.local/state`              | `~/Library/Application Support`   | `%LOCALAPPDATA%`      |

We're currently using the data directory (`$XDG_DATA_HOME/wasm`), and are planning to use the config directory in the future.

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

Currently, we don't support any configuration options. However, we are planning to in the future, see [issue #62](https://github.com/yoshuawuyts/wasm/issues/62).

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
