# Add a Manifest Format

## Summary

Before implementing a manifest format for WASM components in this project, we need to understand the existing ecosystem and make informed decisions about which format to adopt or how to design our own.

## Research Findings

### Existing Manifest Formats for WASM Components

There are two main approaches in the WebAssembly component ecosystem:

#### 1. wit-deps (deps.toml)

**Location:** `wit/deps.toml`

**Purpose:** Dependency manager for WIT (Wasm Interface Types) definitions.

**Format Specification:**
```toml
# Simple URL specifications (downloads and extracts WIT files)
clocks     = "https://github.com/WebAssembly/wasi-clocks/archive/main.tar.gz"
http       = "https://github.com/WebAssembly/wasi-http/archive/main.tar.gz"

# Pin to a specific version/tag
io         = "https://github.com/rvolosatovs/wasi-io/archive/v0.1.0.tar.gz"

# Local path dependency
mywit      = "./path/to/my/wit"

# Table syntax for custom configuration & integrity checks
[keyvalue]
url    = "https://github.com/WebAssembly/wasi-keyvalue/archive/6f3bd6bca07cb7b25703a13f633e05258d56a2dc.tar.gz"
sha256 = "1755b8f1e9f2e70d0bde06198bf50d12603b454b52bf1f59064c1877baa33dff"
sha512 = "7bc43665a9de73ec7bef075e32f67ed0ebab04a1e..."
```

**Key Features:**
- Simple URL or path-based dependency specification
- Optional integrity checking via sha256/sha512
- Dependencies are extracted to `wit/deps/` directory
- Package names inferred from directory structure

**Limitations:**
- No versioning support (uses URLs with version tags)
- No registry integration
- Package name conflicts possible with same-named dependencies
- Local only, not designed for distribution

**Repository:** https://github.com/bytecodealliance/wit-deps

---

#### 2. wkg (wkg.toml)

**Location:** `wkg.toml` or embedded in project manifest

**Purpose:** Declarative registry-oriented package management for WASM components and WIT packages.

**Format Specification:**
```toml
# Default registry for unresolved packages
default_registry = "myorg.registry.com"

# Namespace to registry mapping
[registries]
"wasi" = "wasi.dev"
"myorg" = "myorg.registry.com"

# WIT dependencies with version constraints
[dependencies]
"wasi:http" = "0.1.0"
"custom:logging" = { version = "0.2.0", registry = "myorg.registry.com" }

# Package metadata for publishing
[package]
name = "myorg:example-component"
version = "0.1.0"
description = "My example WASM component"
author = "Jane Doe <jane@example.com>"
licenses = "Apache-2.0"
source = "https://github.com/myorg/example-component"
```

**Key Features:**
- Declarative version constraints
- Multiple registry support with namespace mapping
- Per-package registry overrides
- Full OCI registry integration
- Package metadata for publishing/discovery
- Conflict-free through namespacing and versioning

**Repository:** https://github.com/bytecodealliance/wasm-pkg-tools

---

### Comparison

| Feature              | wit-deps (deps.toml)      | wkg (wkg.toml)                |
|----------------------|---------------------------|-------------------------------|
| **Type**             | Directory structure       | Explicit configuration file   |
| **Dependency Naming**| Inferred from folders     | Package names with versions   |
| **Versioning**       | Not supported             | Fully supported               |
| **Registry Support** | None, local only          | Full OCI registry support     |
| **Conflict Handling**| Conflicts if names collide| Namespaced, avoids collision  |
| **Ease of Use**      | Simple for local dev      | Better for distribution       |
| **Integrity Checks** | SHA256/SHA512             | Registry-based verification   |
| **Publishing**       | Not supported             | Full publish support          |

---

## Recommendations

Based on the research, we recommend adopting the **wkg.toml format** as the basis for our manifest format for the following reasons:

1. **Registry Integration:** Our package manager already supports OCI registries (as seen in `crates/package-manager/src/network/client.rs`), making wkg.toml a natural fit.

2. **Versioning:** Proper semantic versioning support is essential for a package management ecosystem.

3. **Future-Proof:** The wkg format is actively maintained by the Bytecode Alliance and is becoming the de-facto standard for the WASM component ecosystem.

4. **Metadata Support:** The package metadata section aligns well with our existing package management capabilities.

### Proposed Implementation Path

1. **Phase 1:** Add support for parsing `wkg.toml` manifests
   - Add `toml` crate (with `serde` feature) for parsing, paired with existing `serde` dependency
   - Create manifest parsing module in `wasm-package-manager`
   - Support `[dependencies]` section for specifying WIT dependencies

2. **Phase 2:** Integrate with existing package operations
   - Use manifest dependencies in `wasm package pull` operations
   - Add `wasm package init` to create new manifests

3. **Phase 3:** Add publishing support
   - Support `[package]` metadata for `wasm package push`
   - Add manifest validation before publishing

## Tasks

- [ ] Add `toml` crate with `serde` feature for TOML parsing
- [ ] Create manifest module with data structures for wkg.toml format
- [ ] Implement manifest parsing and validation
- [ ] Add CLI command for manifest initialization (`wasm manifest init`)
- [ ] Integrate manifest with existing package pull/push operations
- [ ] Add documentation for manifest format
- [ ] Add tests for manifest parsing

## References

- [Bytecode Alliance wasm-pkg-tools](https://github.com/bytecodealliance/wasm-pkg-tools)
- [Bytecode Alliance wit-deps](https://github.com/bytecodealliance/wit-deps)
- [Component Model Distribution Docs](https://component-model.bytecodealliance.org/composing-and-distributing/distributing.html)
- [WIT Reference](https://component-model.bytecodealliance.org/design/wit.html)

## Acceptance Criteria

- [ ] Research document completed with format comparison
- [ ] Recommendation made for which format to adopt
- [ ] Implementation tasks identified for follow-up work
