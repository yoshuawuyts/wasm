# wasm-manifest Specification

This document defines the requirements for the `wasm-manifest` library crate.
Requirements are derived from the existing test suite.

## Manifest Parsing

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

r[manifest.parse.all-dependencies]
Iterating all dependencies MUST yield both component and interface entries.

r[manifest.parse.permissions]
The manifest parser MUST support sandbox permissions in explicit format
dependencies.

r[manifest.parse.no-permissions]
Dependencies without permissions MUST still parse correctly.

## Manifest Serialization

r[manifest.serialize.compact]
The manifest serializer MUST produce valid TOML in compact format.

r[manifest.serialize.explicit]
The manifest serializer MUST produce valid TOML in explicit format.

## Lockfile

r[lockfile.parse]
The lockfile parser MUST handle TOML lockfiles with version and packages.

r[lockfile.serialize]
The lockfile serializer MUST produce valid TOML output.

r[lockfile.no-dependencies.parse]
Parsing packages without dependencies MUST succeed.

r[lockfile.no-dependencies.serialize]
Serializing packages without dependencies MUST produce valid output.

r[lockfile.mixed-types.parse]
The lockfile MUST support both component and interface package types.

r[lockfile.mixed-types.all-packages]
Iterating all packages MUST yield both component and interface entries.

## Validation

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

## Permissions

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
