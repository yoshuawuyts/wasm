# Contributing
Contributions include code, documentation, answering user questions, running the
project's infrastructure, and advocating for all types of users.

The project welcomes all contributions from anyone willing to work in good faith
with other contributors and the community. No contribution is too small and all
contributions are valued.

This guide explains the process for contributing to the project's GitHub
Repository.

- [Code of Conduct](#code-of-conduct)
- [Bad Actors](#bad-actors)

## Code of Conduct
The project has a [Code of Conduct](./CODE_OF_CONDUCT.md) that *all*
contributors are expected to follow. This code describes the *minimum* behavior
expectations for all contributors.

As a contributor, how you choose to act and interact towards your
fellow contributors, as well as to the community, will reflect back not only
on yourself but on the project as a whole. The Code of Conduct is designed and
intended, above all else, to help establish a culture within the project that
allows anyone and everyone who wants to contribute to feel safe doing so.

Should any individual act in any way that is considered in violation of the
[Code of Conduct](./CODE_OF_CONDUCT.md), corrective actions will be taken. It is
possible, however, for any individual to *act* in such a manner that is not in
violation of the strict letter of the Code of Conduct guidelines while still
going completely against the spirit of what that Code is intended to accomplish.

Open, diverse, and inclusive communities live and die on the basis of trust.
Contributors can disagree with one another so long as they trust that those
disagreements are in good faith and everyone is working towards a common
goal.

## Bad Actors
All contributors to tacitly agree to abide by both the letter and
spirit of the [Code of Conduct](./CODE_OF_CONDUCT.md). Failure, or
unwillingness, to do so will result in contributions being respectfully
declined.

A *bad actor* is someone who repeatedly violates the *spirit* of the Code of
Conduct through consistent failure to self-regulate the way in which they
interact with other contributors in the project. In doing so, bad actors
alienate other contributors, discourage collaboration, and generally reflect
poorly on the project as a whole.

Being a bad actor may be intentional or unintentional. Typically, unintentional
bad behavior can be easily corrected by being quick to apologize and correct
course *even if you are not entirely convinced you need to*. Giving other
contributors the benefit of the doubt and having a sincere willingness to admit
that you *might* be wrong is critical for any successful open collaboration.

Don't be a bad actor.

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
2. **Keep snapshots readable**: Snapshot files are stored in `crates/cli/tests/snapshots/` and are version-controlled. Keep the rendered output clean and readable.
3. **Test different states**: Include tests for empty states, populated states, and interactive states (e.g., filter active, search active).
