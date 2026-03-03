# WIT Parsing

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
