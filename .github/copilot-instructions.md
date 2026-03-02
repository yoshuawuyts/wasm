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
- `cargo xtask sql check` - Verifies database migrations are in sync with schema.sql

**Do not push changes if any of these checks fail.** Fix all formatting issues, clippy warnings, and test failures first.

## Code Style

- Follow Rust idioms and best practices
- All public items should have documentation
- Use `#[must_use]` where appropriate
- Prefer `expect()` over `unwrap()` with descriptive messages

## Database Schema Changes

When changing the database schema, edit `crates/wasm-package-manager/src/storage/schema.sql`
then run `cargo xtask sql migrate --name <description>`. Never hand-write migration files.

Run `cargo xtask sql check` (or `cargo xtask test`) to verify migrations are in sync
before pushing.

## Spec Coverage (Tracey)

This project tracks traceability between specification requirements and code using
[Tracey](https://github.com/bearcove/tracey). The spec files live in `spec/` and the
Tracey configuration is at `.config/tracey/config.styx`.

### Annotation Syntax

The prefix for this project is `r`. Use `r[VERB req.id]` comments to link code to requirements:

```rust
// r[impl some.requirement]
fn implementing_function() { ... }

// r[verify some.requirement]
#[test]
fn test_that_verifies_requirement() { ... }
```

Supported verbs:

| Verb | Meaning |
|------|---------|
| `impl` | This code implements the requirement (default if verb omitted) |
| `verify` | This test verifies the requirement |
| `depends` | This code must be reviewed if the requirement changes |
| `related` | Loosely connected to the requirement |

### When to Add Annotations

- When **implementing** a requirement from `spec/`: add `// r[impl req.id]` above the function
- When **writing a test** that verifies a requirement: add `// r[verify req.id]` above the test
- Spec requirements are defined in `spec/**/*.md` using the `r[req.id]` syntax

### Checking Coverage

The `tracey` binary is available in the agent environment. Use it to find uncovered requirements:

```bash
tracey query uncovered   # requirements without implementation
tracey query untested    # requirements without tests
tracey query validate    # check for broken or stale references
tracey query status      # overall coverage overview
```
