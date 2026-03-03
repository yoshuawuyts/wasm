# Manifest

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
