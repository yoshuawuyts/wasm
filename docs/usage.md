# Usage Guide

This guide covers basic usage patterns for `wasm(1)`, a unified developer tool for WebAssembly.

## Installation

### From crates.io

```bash
cargo install wasm
```

### As a Library

```bash
cargo add wasm
```

## Command Overview

`wasm` provides several command categories:

- **Interactive TUI**: Launch with no arguments
- **Package Management**: Pull, push, and list packages
- **Local Discovery**: Detect and manage local WASM files
- **Inspection**: Examine WASM component structure
- **Self Management**: Configure and manage the tool itself

## Interactive Mode

Launch the interactive terminal user interface:

```bash
wasm
```

The TUI provides:
- Package browsing and search
- Interactive package details
- Keyboard navigation
- Visual package management

**Note**: The TUI only launches when running in an interactive terminal (not when piped or redirected).

## Package Management

### Pulling Packages

Download a package from a registry:

```bash
# Pull from GitHub Container Registry
wasm package pull ghcr.io/example/my-component:latest

# Pull from Docker Hub
wasm package pull myuser/my-component:v1.0.0

# Pull from a custom registry
wasm package pull registry.example.com/org/component:tag
```

The package is stored locally in content-addressable storage and can be listed with `wasm package list`.

### Pushing Packages

Push a local package to a registry:

```bash
wasm package push ghcr.io/myuser/my-component:v1.0.0
```

**Note**: You must be authenticated to push packages. See [Authentication](authentication.md) for details.

### Listing Packages

View all locally stored packages:

```bash
wasm package list
```

This shows:
- Registry and repository
- Tags
- Digests
- Pull timestamps
- Storage size

## Local WASM File Discovery

### Listing Local Files

Detect WASM files in the current directory:

```bash
wasm local list
```

This recursively scans for `.wasm` files and displays:
- File paths
- File sizes
- Component type (if applicable)

The detector respects `.gitignore` rules and standard ignore patterns.

## Inspecting WASM Components

### Basic Inspection

Examine a WASM component file:

```bash
wasm inspect file.wasm
```

This displays:
- Component structure
- Imports and exports
- Metadata
- Dependencies

### Detailed Information

For more detailed information, the inspect command shows:
- Component type information
- Interface definitions
- World descriptions
- Custom sections

## Self Management

### Viewing State

Check storage location and usage:

```bash
wasm self state
```

Displays:
- Executable location
- Data directory paths
- Storage sizes
- Migration status

### Cleaning Storage

Clean up unused content and optimize storage:

```bash
wasm self clean
```

This operation:
- Removes orphaned content
- Vacuums the database
- Reclaims disk space

## Common Workflows

### Exploring a Registry

1. Search for packages (coming soon) or use the TUI
2. Pull interesting packages to inspect them
3. Examine with `wasm inspect` or the TUI

### Publishing a Package

1. Build your WASM component
2. Authenticate with your registry (see [Authentication](authentication.md))
3. Push with `wasm package push registry.example.com/myorg/component:v1.0.0`

### Managing Local Development

1. Use `wasm local list` to discover WASM files in your project
2. Inspect components with `wasm inspect`
3. Test components locally before publishing

### Cleaning Up After Development

1. Run `wasm self state` to check storage usage
2. Remove unused packages manually or with future commands
3. Run `wasm self clean` to reclaim space

## Package Reference Format

Packages are referenced using OCI-style references:

```
[registry/]repository[:tag|@digest]
```

### Examples

```bash
# Full reference with registry and tag
ghcr.io/owner/repo:latest

# Docker Hub shorthand (owner/repo implies docker.io registry)
owner/repo:v1.0.0

# With digest instead of tag
ghcr.io/owner/repo@sha256:abcd1234...

# Custom registry with port
localhost:5000/myrepo:dev
```

### Registry Resolution

- If no registry is specified, Docker Hub (`index.docker.io`) is assumed
- Common registries: `ghcr.io`, `docker.io`, `mcr.microsoft.com`, `quay.io`
- Private registries require full domain specification

## Command-Line Help

Each command and subcommand has built-in help:

```bash
# Top-level help
wasm --help

# Subcommand help
wasm package --help
wasm package pull --help

# Self commands
wasm self --help
```

## Exit Codes

- `0`: Success
- `1`: General error
- `101`: Authentication error
- `102`: Network error
- Other non-zero: Specific error conditions

## Tips and Tricks

### Tab Completion

Generate shell completions (coming soon):

```bash
wasm self completions bash > ~/.local/share/bash-completion/completions/wasm
```

### Quick Package Inspection

Combine commands to quickly pull and inspect:

```bash
wasm package pull ghcr.io/example/component:latest
wasm inspect ~/.local/share/wasm/store/content/<digest>
```

### Finding Package Content

After pulling a package, use `wasm package list` to find its digest, then access content in the store directory.

### Using with CI/CD

In CI/CD pipelines:

1. Authenticate using `docker login` or similar
2. Use `wasm package pull` to retrieve dependencies
3. Use `wasm package push` to publish artifacts
4. Use `wasm self clean` to manage storage between builds

## Troubleshooting

### Package Not Found

If pulling fails with "not found":
- Verify the package reference is correct
- Check authentication (see [Authentication](authentication.md))
- Ensure the package exists and is accessible

### Storage Issues

If you encounter storage errors:
- Run `wasm self state` to check space
- Run `wasm self clean` to free up space
- Check filesystem permissions on `~/.local/share/wasm`

### Network Errors

For network-related failures:
- Check internet connectivity
- Verify registry is accessible
- Check firewall and proxy settings

## Further Reading

- [Authentication](authentication.md) - Set up registry access
- [Configuration](configuration.md) - Understand storage and settings
- [API Documentation](https://docs.rs/wasm) - Library usage

## Getting Help

- GitHub Issues: [https://github.com/yoshuawuyts/wasm/issues](https://github.com/yoshuawuyts/wasm/issues)
- Command help: `wasm --help`
- This documentation: `/docs` directory
