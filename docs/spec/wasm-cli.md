# wasm-cli Specification

This document defines the requirements for the `wasm` CLI tool and its supporting
crates. Requirements are derived from the existing test suite and serve as a
traceability baseline for spec coverage.

## CLI

The `wasm` command-line interface provides subcommands for managing WebAssembly
components and interfaces.

### Help and Version

r[cli.help]
The CLI MUST provide `--help` output for every command and subcommand.

r[cli.version]
The CLI MUST print a version string containing the program name when invoked
with `--version`.

### Color Support

r[cli.color.flag]
The CLI MUST accept a `--color` flag with values `auto`, `always`, and `never`.

r[cli.color.invalid]
The CLI MUST reject invalid `--color` values with an error message.

r[cli.color.no-color-env]
The CLI MUST respect the `NO_COLOR` environment variable.

r[cli.color.clicolor-env]
The CLI MUST respect the `CLICOLOR` environment variable.

r[cli.color.subcommand]
The `--color` flag MUST work when combined with subcommands.

### Offline Mode

r[cli.offline.flag]
The CLI MUST accept an `--offline` flag.

r[cli.offline.registry-blocked]
When `--offline` is set, registry operations MUST fail with a clear error
mentioning offline mode.

r[cli.offline.local-allowed]
When `--offline` is set, local operations MUST still succeed.

### Shell Completions

r[cli.completions.bash]
The CLI MUST generate valid Bash completions.

r[cli.completions.zsh]
The CLI MUST generate valid Zsh completions.

r[cli.completions.fish]
The CLI MUST generate valid Fish completions.

r[cli.completions.invalid]
The CLI MUST reject invalid shell names for completions.

### Man Pages

r[cli.man-pages]
The CLI MUST generate man pages that reference the program name.

## Init Command

The `init` subcommand scaffolds a new project directory.

r[init.current-dir]
Running `wasm init` without arguments MUST create files in the current directory.

r[init.explicit-path]
Running `wasm init <path>` MUST create files at the specified path.

r[init.directory-structure]
The init command MUST create `deps/vendor/wit` and `deps/vendor/wasm` directories.

r[init.manifest]
The init command MUST create a valid TOML manifest at `deps/wasm.toml` containing
a `components` or `interfaces` table.

r[init.lockfile]
The init command MUST create a valid lockfile at `deps/wasm.lock.toml` with a
lockfile version and auto-generation comments.

## Run Command

The `run` subcommand executes a WebAssembly component.

r[run.core-module-rejected]
The run command MUST reject core WebAssembly modules with a clear error message.

r[run.missing-file]
The run command MUST report a clear error when the target file does not exist.

## Dotenv

The CLI supports loading environment variables from `.env` files.

r[dotenv.detection]
The CLI MUST detect and report the presence of a `.env` file in `self config`
output, including the number of variables defined.

r[dotenv.not-found]
When no `.env` file exists, the CLI MUST report it as `not found`.

r[dotenv.loading]
The CLI MUST load variables from a `.env` file successfully.

r[dotenv.precedence]
System environment variables MUST take precedence over `.env` file variables.

## Wasm Detector

The `wasm-detector` crate finds `.wasm` files on the local filesystem.

r[detector.find-wasm]
The detector MUST find all `.wasm` files in a directory tree.

r[detector.target-dir]
The detector MUST find build artifacts in `target/wasm32-*` directories.

r[detector.pkg-dir]
The detector MUST find wasm-pack output in `pkg/` directories.

r[detector.dist-dir]
The detector MUST find jco/JavaScript output in `dist/` directories.

r[detector.entry-methods]
A `WasmEntry` MUST expose the file path and file name.

r[detector.gitignore]
The detector MUST respect `.gitignore` while still including well-known
directories such as `target`, `pkg`, and `dist`.

r[detector.empty-dir]
The detector MUST handle empty directories gracefully, returning no results.

r[detector.convenience]
The `detect()` convenience method MUST return the same results as the iterator.

## Manifest

The `wasm-manifest` crate handles manifest and lockfile parsing.

### Manifest Parsing

r[manifest.parse.compact]
The manifest parser MUST support compact dependency notation.

r[manifest.parse.explicit]
The manifest parser MUST support explicit table dependency notation with
registry, namespace, package, and version fields.

r[manifest.parse.empty]
The manifest parser MUST handle empty manifest files.

r[manifest.parse.mixed]
The manifest parser MUST support manifests with both `components` and
`interfaces` sections.

r[manifest.parse.permissions]
The manifest parser MUST support sandbox permissions in explicit format
dependencies.

r[manifest.parse.no-permissions]
Dependencies without permissions MUST still parse correctly.

### Manifest Serialization

r[manifest.serialize.compact]
The manifest serializer MUST produce valid TOML in compact format.

r[manifest.serialize.explicit]
The manifest serializer MUST produce valid TOML in explicit format.

### Lockfile

r[lockfile.parse]
The lockfile parser MUST handle TOML lockfiles with version and packages.

r[lockfile.serialize]
The lockfile serializer MUST produce valid TOML output.

r[lockfile.no-dependencies]
Packages without dependencies MUST be handled correctly.

r[lockfile.mixed-types]
The lockfile MUST support both component and interface package types.

### Validation

r[validation.success]
Validation MUST pass when manifest and lockfile are consistent.

r[validation.missing-dependency]
Validation MUST detect packages in the lockfile that are not in the manifest.

r[validation.invalid-dependency]
Validation MUST detect package dependencies referencing non-existent packages.

r[validation.empty]
Validation MUST pass for empty manifest and lockfile pairs.

r[validation.error-display]
Validation errors MUST have human-readable display messages.

r[validation.mixed-types]
Validation MUST handle both component and interface sections.

### Permissions

r[permissions.defaults]
Default permissions MUST resolve to correct values.

r[permissions.merge]
Permission merge MUST properly override fields from the base.

r[permissions.merge-preserve]
Permission merge MUST preserve base values when override is `None`.

r[permissions.serde]
Permissions MUST survive a serialization/deserialization roundtrip.

r[permissions.toml]
Permissions MUST be deserializable from TOML fragments.

## Configuration

The `config` module manages global and local configuration files.

r[config.default]
A default configuration MUST be constructable.

r[config.load-missing]
Loading a nonexistent config file MUST succeed gracefully.

r[config.load-valid]
Loading a valid config file MUST return the correct settings.

r[config.ensure-exists]
`ensure_exists` MUST create the config file if it is missing.

r[config.ensure-idempotent]
`ensure_exists` MUST be idempotent.

r[config.credentials.cache]
Credential caching MUST work correctly.

r[config.credentials.no-helper]
Missing credential helpers MUST be handled gracefully.

r[config.local-overrides]
Local configuration MUST override global configuration.

## Credential Helper

The credential helper subsystem extracts credentials for OCI registries.

r[credential.json]
JSON credential helpers MUST be executed and parsed correctly.

r[credential.split]
Split credential helpers MUST be executed correctly.

r[credential.no-leak-debug]
Debug output MUST never print credentials.

r[credential.no-leak-display]
Display output MUST never leak credentials.

## OCI Storage

The OCI storage layer persists OCI registry data in SQLite.

### Repository and Manifest

r[oci.repository.upsert]
Upserting an OCI repository MUST be idempotent.

r[oci.manifest.upsert]
Upserting an OCI manifest MUST store and retrieve correctly.

r[oci.manifest.annotations]
Manifest upsert MUST extract and store annotations.

r[oci.manifest.config-fields]
Manifest upsert MUST store config fields.

r[oci.manifest.placeholder-upgrade]
Upserting a manifest over a placeholder MUST upgrade it with full data.

r[oci.manifest.cascade-delete]
Deleting a manifest MUST cascade to layers, annotations, and referrers.

### Tags

r[oci.tag.upsert]
Upserting an OCI tag MUST be idempotent.

### Layers

r[oci.layer.insert]
Inserting OCI layers MUST allow listing them afterward.

r[oci.layer.annotations]
Layer annotations MUST be insertable and listable.

r[oci.layer.annotation-conflict]
Layer annotation upsert MUST handle conflicts.

r[oci.layer.annotation-cascade]
Deleting a layer MUST cascade to its annotations.

### Referrers

r[oci.referrer.insert]
OCI referrers MUST be insertable and listable.

r[oci.referrer.idempotent]
Referrer insertion MUST be idempotent.

r[oci.referrer.cascade-delete]
Deleting a manifest MUST cascade to its referrer relationships.

### Tag Classification

r[oci.tags.classify-release]
Release tags MUST be classified correctly.

r[oci.tags.classify-signature]
Signature tags MUST be classified correctly.

r[oci.tags.classify-attestation]
Attestation tags MUST be classified correctly.

r[oci.tags.classify-mixed]
Mixed tag lists MUST be classified correctly.

r[oci.tags.classify-empty]
Empty tag lists MUST be classified correctly.

### Layer Filtering

r[oci.layers.filter-mixed]
Filtering MUST separate WASM layers from non-WASM layers.

r[oci.layers.filter-none]
Filtering MUST handle layers with no WASM content.

r[oci.layers.filter-empty]
Filtering MUST handle an empty layer list.

### Orphaned Layers

r[oci.layers.orphaned-disjoint]
Orphaned layer detection MUST work with disjoint layer sets.

r[oci.layers.orphaned-overlap]
Orphaned layer detection MUST work with overlapping layer sets.

r[oci.layers.orphaned-shared]
Orphaned layer detection MUST handle all-shared layers.

## WIT Storage

The WIT metadata storage layer persists WebAssembly Interface Types data.

r[wit.world.insert]
WIT worlds MUST be insertable and queryable.

r[wit.world.imports-exports]
WIT world imports and exports MUST be storable.

r[wit.world.idempotent]
Import and export operations MUST be idempotent.

r[wit.interface.dependencies]
WIT interface dependencies MUST be storable.

r[wit.component.insert]
WASM components and their targets MUST be storable.

r[wit.component.wit-only]
WIT-only packages MUST NOT create component rows.

### Foreign Key Resolution

r[wit.resolve.import]
Import resolution MUST populate `resolved_interface_id` when the dependency exists.

r[wit.resolve.import-missing]
Import resolution MUST leave the field NULL when the dependency is missing.

r[wit.resolve.dependency]
Dependency interface IDs MUST be resolvable.

r[wit.resolve.export]
Export interface IDs MUST be resolvable.

r[wit.resolve.component-target]
Component targets MUST be resolvable across packages.

## WIT Parsing

The WIT parser extracts interface metadata from WASM binaries.

r[wit.parse.invalid-bytes]
The parser MUST return `None` for invalid bytes.

r[wit.parse.empty-bytes]
The parser MUST return `None` for empty bytes.

r[wit.parse.core-module]
The parser MUST handle core WASM modules.

r[wit.parse.random-bytes]
The parser MUST return `None` for random data.

r[wit.parse.world-key-name]
World key names MUST be converted correctly.

r[wit.parse.world-key-interface]
Interface world keys MUST be converted correctly.

r[wit.parse.wit-text-package]
WIT text generation MUST work for WIT packages.

r[wit.parse.wit-text-component]
WIT text generation MUST work for components.

r[wit.parse.wit-text-imports-exports]
WIT text generation MUST include imports and exports.

r[wit.parse.multiple-worlds]
Extraction MUST handle packages with multiple worlds.

r[wit.parse.single-world]
Components MUST have exactly one world.

r[wit.parse.world-items]
World items with named and interface imports MUST be extracted.

r[wit.parse.exclude-primary]
Dependencies MUST exclude the primary package itself.

r[wit.parse.is-component]
The `is_component` flag MUST correctly distinguish WIT packages from components.

## WIT Detection

r[wit.detect.invalid]
Invalid bytes MUST NOT be detected as a WIT package.

r[wit.detect.empty]
Empty bytes MUST NOT be detected as a WIT package.

r[wit.detect.core-module]
Core modules MUST NOT be detected as WIT packages.

## Package Manager Logic

### Vendor Filenames

r[manager.vendor-filename.basic]
Vendor filenames MUST be generated from registry, repository, tag, and digest.

r[manager.vendor-filename.no-tag]
Vendor filenames MUST handle missing tags.

r[manager.vendor-filename.short-digest]
Vendor filenames MUST handle short digest lengths.

r[manager.vendor-filename.nested]
Vendor filenames MUST handle nested repository paths.

### Sync Scheduling

r[manager.sync.no-previous]
Sync MUST trigger when there is no previous sync time.

r[manager.sync.stale]
Sync MUST trigger when the sync interval has expired.

r[manager.sync.fresh]
Sync MUST NOT trigger when the sync interval has not expired.

### Name Derivation

r[manager.name.wit-package]
Name derivation MUST prefer the WIT package name.

r[manager.name.oci-title]
Name derivation MUST fall back to the OCI image title.

r[manager.name.last-segment]
Name derivation MUST fall back to the repository last segment.

r[manager.name.collision]
Name derivation MUST handle collisions.

r[manager.name.sanitize]
Name derivation MUST sanitize invalid identifier characters.

## Database

### Migrations

r[db.migrations.create-tables]
Running all migrations MUST create the required database tables.

r[db.migrations.idempotent]
Running migrations MUST be idempotent.

r[db.migrations.info]
Migration info MUST be retrievable.

### Known Packages

r[db.known-packages.upsert]
Upserting a known package MUST work for both new and existing entries.

r[db.known-packages.search]
Known package search MUST return matching results.

r[db.known-packages.search-empty]
Known package search MUST handle no results gracefully.

r[db.known-packages.reference]
Known package reference strings MUST be generated correctly.

## TUI

The terminal user interface renders views using `ratatui`.

### Views

r[tui.local-view]
The local view MUST render empty and populated states.

r[tui.interfaces-view]
The interfaces view MUST render empty and populated states.

r[tui.packages-view]
The packages view MUST render empty, populated, and filtered states.

r[tui.package-detail-view]
The package detail view MUST render full metadata and handle missing tags.

r[tui.search-view]
The search view MUST render empty, populated, and active search states.

r[tui.known-package-detail-view]
The known package detail view MUST render full and minimal metadata.

r[tui.settings-view]
The settings view MUST render loading and populated states.

r[tui.log-view]
The log view MUST render empty, populated, and scrolled states.

### Components

r[tui.tab-bar]
The tab bar MUST render selected tabs, loading, and error states correctly.

## Formatting

r[format.size]
The `format_size` function MUST format byte sizes in human-readable units
(bytes, KB, MB, GB).
