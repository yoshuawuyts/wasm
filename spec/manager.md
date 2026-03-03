# Package Manager

## Vendor Filenames

r[manager.vendor-filename.basic]
Vendor filenames MUST be generated from registry, repository, tag, and digest.

r[manager.vendor-filename.no-tag]
Vendor filenames MUST handle missing tags.

r[manager.vendor-filename.short-digest]
Vendor filenames MUST handle short digest lengths.

r[manager.vendor-filename.nested]
Vendor filenames MUST handle nested repository paths.

## Sync Scheduling

r[manager.sync.no-previous]
Sync MUST trigger when there is no previous sync time.

r[manager.sync.stale]
Sync MUST trigger when the sync interval has expired.

r[manager.sync.fresh]
Sync MUST NOT trigger when the sync interval has not expired.

## Name Sanitization

r[manager.name.sanitize.valid]
A valid identifier MUST pass through unchanged.

r[manager.name.sanitize.uppercase]
Uppercase characters MUST be lowercased.

r[manager.name.sanitize.underscores]
Underscores MUST be replaced with hyphens.

r[manager.name.sanitize.leading-digits]
Leading digits MUST be stripped.

## Name Derivation

r[manager.name.wit-package]
Name derivation MUST prefer the WIT package name.

r[manager.name.oci-title]
Name derivation MUST fall back to the OCI image title.

r[manager.name.last-segment]
Name derivation MUST fall back to the repository last segment.

r[manager.name.collision]
Name derivation MUST handle collisions.
