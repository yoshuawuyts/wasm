# Configuration

`wasm(1)` uses a local storage system to manage downloaded packages and metadata. This guide explains the storage layout and configuration options.

## Storage Location

By default, `wasm` stores all data in your system's local data directory:

### Linux and macOS

```
~/.local/share/wasm/
```

### Windows

```
%LOCALAPPDATA%\wasm\
```

The exact location is determined by the [`dirs`](https://docs.rs/dirs/) crate, which follows platform conventions.

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

The store directory uses [`cacache`](https://docs.rs/cacache/) for content-addressable storage:

- **Immutable**: Content is stored by its SHA-256 hash
- **Deduplicated**: Identical content is stored only once
- **Efficient**: Supports concurrent access and integrity verification
- **OCI-Compatible**: Stores image layers and manifests following OCI specifications

#### Metadata Database (`db/metadata.db3`)

The metadata database is a SQLite database that stores:

- Package references (registry, repository, tag)
- Digest mappings to content in the store
- Known packages for discovery
- Tag classifications (primary, signature, attestation)
- Timestamps for pull operations

### Database Schema

The database includes several tables (managed through migrations):

- `images`: Core package metadata (registry, repository, digest)
- `known_packages`: Discovered packages and their tags
- `migrations`: Schema version tracking

## Configuration Options

### Inspecting Configuration

Use the `wasm self state` command to view current configuration:

```bash
wasm self state
```

This displays:
- Executable location
- Data directory path
- Store directory path and size
- Metadata file path and size
- Migration version information

### Example Output

```
Executable:      /usr/local/bin/wasm
Data Directory:  /home/user/.local/share/wasm
Store Directory: /home/user/.local/share/wasm/store (125.3 MB)
Metadata File:   /home/user/.local/share/wasm/db/metadata.db3 (256 KB)
Migrations:      3/3 (up to date)
```

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

## Advanced Configuration

### Custom Storage Location

Currently, the storage location is determined automatically by platform conventions. Custom locations may be supported in future versions.

### Environment Variables

No environment variables are currently used for configuration. Configuration is managed through the storage system and command-line options.

## Performance Considerations

### Concurrent Access

- The SQLite database uses Write-Ahead Logging (WAL) for better concurrency
- Content-addressable store supports parallel reads and writes
- Multiple `wasm` processes can safely access the storage simultaneously

### Storage Growth

- Content is never deleted automatically (except via `wasm self clean`)
- Each unique package version and layer is stored separately
- Similar packages may share layers (deduplication)
- Regular cleanup is recommended for active users

## Backup and Migration

### Backing Up Data

To back up your `wasm` data:

```bash
# Back up the entire data directory
cp -r ~/.local/share/wasm ~/.local/share/wasm.backup
```

### Moving to a New System

1. Copy the data directory to the new system
2. Ensure it's in the correct platform-specific location
3. Run `wasm self state` to verify

### Recovering from Corruption

If the database becomes corrupted:

1. Back up the data directory
2. Run `wasm self clean` to attempt automatic repair
3. If issues persist, delete `db/metadata.db3` (you'll need to re-pull packages)

The content store will remain intact even if the database is deleted, though package references will be lost.

## Security Considerations

### File Permissions

- The storage directory is created with user-only permissions
- Ensure `~/.local/share/wasm` has appropriate permissions (700 recommended)

### Content Integrity

- All content is verified using SHA-256 checksums
- Corrupted content is detected and rejected
- Registry signatures can be stored as separate package tags

## Future Configuration Options

Planned configuration enhancements:
- Custom storage locations via config file or environment variables
- Storage quotas and automatic cleanup policies
- Configurable cache expiration
- Proxy and network settings
- Logging and debugging options
