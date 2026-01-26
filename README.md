<h1 align="center">wasm</h1>
<div align="center">
  <strong>
    Unified developer tools for WebAssembly
  </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/wasm">
    <img src="https://img.shields.io/crates/v/wasm.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/wasm">
    <img src="https://img.shields.io/crates/d/wasm.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/wasm">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/wasm">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/wasm/releases">
      Releases
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/wasm/blob/master.github/CONTRIBUTING.md">
      Contributing
    </a>
  </h3>
</div>

## Installation
```sh
$ cargo add wasm
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

[contributing]: https://github.com/yoshuawuyts/wasm/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/wasm/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/wasm/labels/help%20wanted

## Snapshot Testing

This project uses the [`insta`](https://crates.io/crates/insta) crate for snapshot testing the TUI views. Snapshot tests help catch unintentional changes in the UI, providing more confidence during refactoring and new feature development.

### Running Snapshot Tests

```sh
# Run all tests including snapshot tests
$ cargo test --package wasm

# Run only snapshot tests
$ cargo test --package wasm snapshot
```

### Updating Snapshots

When views change intentionally, you can update the snapshots:

```sh
# Install the insta CLI tool (first time only)
$ cargo install cargo-insta

# Review pending snapshot changes interactively
$ cargo insta review

# Or automatically accept all new snapshots
$ cargo insta accept
```

Alternatively, you can update snapshots directly during test runs:

```sh
# Accept all new/changed snapshots automatically
$ INSTA_UPDATE=always cargo test --package wasm
```

### Best Practices for Snapshot Tests

1. **Review changes carefully**: Always review snapshot changes before accepting them to ensure they match your expected output.
2. **Keep snapshots readable**: Snapshot files are stored in `crates/cli/src/tui/views/snapshots/` and are version-controlled. Keep the rendered output clean and readable.
3. **Test different states**: Include tests for empty states, populated states, and interactive states (e.g., filter active, search active).
4. **Use consistent test data**: Use the provided helper functions (`create_test_image_entry`, `create_test_known_package`, `create_test_state_info`) to create consistent test data.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
