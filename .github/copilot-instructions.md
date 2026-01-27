## Development Environment

This is a Rust project that uses `xtask` for task automation.

## Before Pushing Any Changes

Always run the following command before committing or pushing changes:

```bash
cargo xtask test
```

This command runs:
- `cargo fmt` - Ensures code is properly formatted
- `cargo clippy` - Runs lints to catch common mistakes
- `cargo test` - Runs the test suite

**Do not push changes if any of these checks fail.** Fix all formatting issues, clippy warnings, and test failures first.

## Code Style

- Follow Rust idioms and best practices
- All public items should have documentation
- Use `#[must_use]` where appropriate
- Prefer `expect()` over `unwrap()` with descriptive messages
