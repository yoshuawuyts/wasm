---
name: spec-driven-development
description: Implement features using spec-driven development — write specs first, then tests, then code. Use when implementing new features, adding behavior, writing tests for requirements, or working with spec files.
---

# Spec-Driven Development

When implementing a feature, follow a strict spec → test → implement → validate
loop. The goal is to end up with equal parts: **rules in prose**, **tests making
those rules validatable**, and **an implementation implementing the rules**.

## When to Use

Use this workflow when:
- Implementing a new feature or behavior
- Adding or modifying requirements in `spec/*.md`
- Writing tests that verify spec requirements
- Working on a PR that changes functionality

## Step 1: Write the Spec

Before writing any code, define the feature's requirements in the appropriate
spec file under `spec/`. Each requirement must use `r[req.id]` syntax, be
written as a normative statement using MUST/MUST NOT/SHOULD, and explain both
**what** the behavior is and **why** it matters.

- Add requirements to the relevant `spec/*.md` file under a clear heading.
- Use dot-separated hierarchical IDs (e.g., `oci.install.local-namespace`).
- Each requirement should be a single, testable rule.
- Group related requirements under a shared section with a brief intro explaining
  the design rationale.

Example:

```markdown
### Local Namespace

Components installed from unregistered sources need a namespace so they can be
referenced consistently. The `local` namespace serves as the default home for
these packages.

r[install.local-namespace.default]
Installing a component without an explicit namespace MUST place it under
the `local` namespace.

r[install.local-namespace.explicit]
Installing a component with an explicit namespace MUST use the provided
namespace instead of `local`.
```

## Step 2: Write Tests

Write tests **before** the implementation. Each test must reference the spec
requirement it verifies using `// r[verify req.id]`. Tests should be minimal
and focused — one test per requirement where possible.

- Place tests in the appropriate `tests/` directory or inline `mod tests`.
- Use `// r[verify req.id]` immediately above each `#[test]` function.
- Test names should describe the expected behavior, not the implementation.
- Tests will fail at this stage — that's expected.

Example:

```rust
// r[verify install.local-namespace.default]
#[test]
fn install_without_namespace_uses_local() {
    let result = install("example.wasm", None);
    assert_eq!(result.namespace(), "local");
}

// r[verify install.local-namespace.explicit]
#[test]
fn install_with_explicit_namespace_uses_it() {
    let result = install("example.wasm", Some("custom"));
    assert_eq!(result.namespace(), "custom");
}
```

## Step 3: Implement

Write the implementation to make the tests pass. Annotate functions with
`// r[impl req.id]` to link them back to the spec.

- Implement the minimum code needed to satisfy the spec requirements.
- Add `// r[impl req.id]` above the implementing function(s).
- Follow existing code style and conventions.

## Step 4: Validate and Iterate

Run the test suite:

```bash
cargo xtask test
```

If tests fail:

1. **Diagnose**: Determine whether the failure is in the spec, test, or
   implementation.
2. **Revise the spec** if the requirement was ambiguous or wrong.
3. **Revise the test** if it doesn't accurately capture the requirement.
4. **Revise the implementation** if the code doesn't match the spec.
5. **Re-run** until all tests pass and `cargo xtask test` succeeds.

Use `tracey` to verify coverage:

```bash
tracey query uncovered   # requirements without implementation
tracey query untested    # requirements without tests
tracey query validate    # check for broken references
```

## Checklist

Before considering the feature complete:

- [ ] Every requirement in the spec has at least one `r[verify ...]` test
- [ ] Every requirement in the spec has at least one `r[impl ...]` annotation
- [ ] `cargo xtask test` passes (fmt, clippy, tests, sql check)
- [ ] `tracey query validate` shows no broken references
