# Add a Manifest Format

## Summary

Before implementing a manifest format for WASM components in this project, we need to understand the existing ecosystem and make informed decisions about which format to adopt or how to design our own.

## Research Findings

### Existing Manifest Formats for WASM Components

There are four main approaches in the WebAssembly component ecosystem:

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

#### 3. JavaScript Import Maps (Web Standard)

**What are JS Import Maps?**

JavaScript import maps are a web standard that allows developers to control how ES module specifiers are resolved in the browser. They map bare import specifiers to actual URLs.

**Example (HTML):**
```html
<script type="importmap">
{
  "imports": {
    "lodash": "https://cdn.skypack.dev/lodash",
    "my-lib/": "/web_modules/my-lib/",
    "./myModule.wasm": "./myModule.abcd1234.wasm"
  }
}
</script>
```

**Can OCI identifiers be used in JS import maps?**

**No, not directly.** Browser import maps only support HTTP(S) URLs. You cannot use `oci://` or OCI registry references directly in JavaScript import maps because:

1. Browsers require valid HTTP(S) URLs for module resolution
2. OCI registries use a different protocol and require authentication/pull operations
3. The OCI artifact format requires unpacking before use

**Workarounds:**

To use OCI-hosted WASM modules with JS import maps, you need an intermediary step:

```javascript
// Option 1: Pre-fetch and serve via HTTP
// Pull from OCI registry at build/deploy time, serve via HTTP
// Then use in import map:
{
  "imports": {
    "./component.wasm": "https://your-cdn.com/component.wasm"
  }
}

// Option 2: Custom loader at runtime
async function loadFromOCI(ociRef) {
  // Resolve OCI reference to HTTP URL via proxy/service
  const httpUrl = await resolveOCIToHttp(ociRef);
  const wasm = await WebAssembly.instantiateStreaming(fetch(httpUrl), imports);
  return wasm;
}
```

**Current Status:**
- The WebAssembly ESM integration proposal allows `import { foo } from "./module.wasm"`
- Import maps can remap `.wasm` file paths to different URLs
- But OCI registry references require server-side or build-time resolution

---

#### 4. wkg Import Maps (WASM Ecosystem)

**Purpose:** Unlike browser import maps, wkg-style import maps are a tooling concept for WASM component dependency resolution that **does support OCI identifiers**.

**Can OCI identifiers be used here?** **Yes.** This is designed specifically for OCI registry integration.

**Format (within wkg configuration):**
```toml
# Default registry for unresolved packages
default_registry = "acme.registry.com"

# Namespace-to-registry mapping (import resolution)
[registries]
"wasi" = "ghcr.io/bytecodealliance/wasi-interfaces"
"myorg" = "ghcr.io/myorg/components"

# Override specific packages to different OCI locations
[overrides]
"wasi:logging" = "ghcr.io/someone/special-logging:1.0.0"
"wasi:key-value" = "ghcr.io/other-registry/key-value:2.0.0"
```

**Key Features:**
- Maps logical import names to physical OCI registry locations
- Namespace-level registry mapping (all `wasi:*` packages from one registry)
- Per-package overrides for specific dependencies
- Full OCI reference support including tags and digests

**Key Difference from JS Import Maps:**
- JS import maps: Browser standard, HTTP(S) URLs only, for ES modules
- wkg import maps: Tooling concept, OCI registry support, for WASM components

**Repository:** Part of https://github.com/bytecodealliance/wasm-pkg-tools

---

### Comparison

| Feature              | wit-deps          | wkg.toml          | JS Import Maps (Web) | wkg Import Maps       |
|----------------------|-------------------|-------------------|----------------------|-----------------------|
| **Type**             | Directory struct  | Config file       | Browser standard     | Tooling config        |
| **Dependency Naming**| Inferred folders  | Package + version | URL-based            | Namespace/package     |
| **Versioning**       | Not supported     | Fully supported   | Via URL paths        | Via OCI tags/digests  |
| **OCI Identifiers**  | No                | Yes               | **No (HTTP only)**   | Yes, native           |
| **Registry Support** | None, local only  | Full OCI          | HTTP(S) only         | Full OCI              |
| **Conflict Handling**| Name collisions   | Namespaced        | URL-based            | Per-package overrides |
| **Environment**      | Build-time        | Build/deploy      | Browser runtime      | Build/deploy tooling  |
| **Publishing**       | Not supported     | Full support      | N/A                  | N/A (resolution only) |

**Key Finding:** JavaScript import maps (the web standard) **cannot use OCI identifiers directly**. They only support HTTP(S) URLs. To use OCI-hosted WASM in browsers, you must resolve OCI references to HTTP URLs at build or deploy time.

---

## Recommendations

Based on the research, we recommend adopting the **wkg.toml format** as the basis for our manifest format for the following reasons:

1. **Registry Integration:** Our package manager already supports OCI registries (as seen in `crates/package-manager/src/network/client.rs`), making wkg.toml a natural fit.

2. **Versioning:** Proper semantic versioning support is essential for a package management ecosystem.

3. **Future-Proof:** The wkg format is actively maintained by the Bytecode Alliance and is becoming the de-facto standard for the WASM component ecosystem.

4. **Metadata Support:** The package metadata section aligns well with our existing package management capabilities.

5. **Import Map Support:** The wkg configuration includes import map functionality through `[registries]` and `[overrides]` sections, allowing OCI identifiers to be used for dependency resolution.

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
- [CNCF Wasm OCI Artifact Layout](https://tag-runtime.cncf.io/wgs/wasm/deliverables/wasm-oci-artifact/)
- [wasm-to-oci (OCI registry distribution)](https://github.com/engineerd/wasm-to-oci)
- [cargo-component Registry and Dependency Management](https://deepwiki.com/bytecodealliance/cargo-component/3.4-registry-and-dependency-management)
- [JavaScript Import Maps - MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script/type/importmap)
- [WebAssembly ESM Integration Proposal](https://github.com/WebAssembly/esm-integration/blob/main/proposals/esm-integration/README.md)
- [JavaScript Import Maps In-Depth (SpiderMonkey)](https://spidermonkey.dev/blog/2023/03/02/javascript-import-maps-part-2-in-depth-exploration.html)

## Acceptance Criteria

- [x] Research document completed with format comparison
- [x] Recommendation made for which format to adopt
- [x] Implementation tasks identified for follow-up work
- [x] Import maps researched and documented (both JS web standard and wkg-style)
- [x] OCI identifier support in import maps documented (JS: No, wkg: Yes)
