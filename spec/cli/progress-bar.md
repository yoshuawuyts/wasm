# Progress Bar

The install command displays a progress bar for each package being downloaded.
The progress bar shows the package name, download progress, and estimated time
remaining. All packages are displayed in a flat tree structure.

## Tree Structure

r[cli.progress-bar.tree-glyph]
The progress display MUST show a tree-like interface with `├──` for non-last
items and `└──` for the last item.

r[cli.progress-bar.no-indent]
The tree interface MUST NOT be indented beyond the tree glyphs.

## Package Name Display

r[cli.progress-bar.namespace-name]
The package MUST be displayed in `namespace:name` form, not the full OCI URL.

r[cli.progress-bar.package-name-downloading]
The package name MUST be displayed in yellow while downloading.

r[cli.progress-bar.package-name-complete]
The package name MUST be displayed in green when downloading completes.

r[cli.progress-bar.version-grey]
The `@<version>` string MUST be displayed in grey.

## Progress Bar Display

r[cli.progress-bar.bar-yellow]
The progress bar MUST be displayed in yellow.

r[cli.progress-bar.bar-hidden-on-complete]
The progress bar MUST be hidden when the download completes.

r[cli.progress-bar.size-grey]
The download size MUST be displayed in grey.

r[cli.progress-bar.eta-grey]
The time remaining MUST be displayed in grey.

r[cli.progress-bar.aggregate-layers]
The progress MUST show the overall progress for the entire package, not
individual layers.
