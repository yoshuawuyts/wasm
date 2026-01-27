# wasm-scanner

A library to scan local `.wasm` files in a repository.

## Features

- Scans for `.wasm` files in a directory
- Respects `.gitignore` rules
- Includes well-known .wasm locations that are typically ignored:
  - `target/wasm32-*/**/*.wasm` (Rust wasm targets)
  - `target/wasm-gc-*/**/*.wasm` (Rust wasm-gc targets)
  - `pkg/**/*.wasm` (wasm-pack output)
  - `dist/**/*.wasm` (JavaScript/jco output)

## Usage

```rust
use wasm_scanner::WasmScanner;
use std::path::Path;

let scanner = WasmScanner::new(Path::new("."));
for result in scanner {
    match result {
        Ok(entry) => println!("Found: {}", entry.path().display()),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Well-known Locations

The scanner automatically includes these typically-ignored directories:

| Location | Description |
|----------|-------------|
| `target/wasm32-*` | Rust wasm32 target outputs |
| `target/wasm-gc-*` | Rust wasm-gc target outputs |
| `pkg/` | wasm-pack output directory |
| `dist/` | JavaScript/jco build output |

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
