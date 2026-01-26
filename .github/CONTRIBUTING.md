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

## Releases

This project uses [release-plz](https://github.com/MarcoIeni/release-plz) to automate the release process for all workspace crates. The release process is fully automated through GitHub Actions and requires no manual intervention for normal releases.

### How Automated Releases Work

1. **Development**: Contributors make changes and submit pull requests as usual
2. **Release PR Creation**: When changes are merged to `main`, release-plz automatically:
   - Analyzes git history to determine version bumps (following [Semantic Versioning](https://semver.org/))
   - Generates/updates CHANGELOG.md for each affected crate
   - Creates a release PR with version updates
3. **Review and Merge**: Maintainers review the release PR and merge it when ready
4. **Automated Publishing**: Once the release PR is merged, release-plz automatically:
   - Publishes all updated crates to [crates.io](https://crates.io)
   - Creates GitHub releases with tags (e.g., `wasm-v0.1.0`)
   - Attaches changelog entries to the GitHub releases

### Manual Release Steps (Exceptional Cases)

In most cases, releases are fully automated. However, if you need to perform a manual release:

1. **Prerequisites**: You need a crates.io API token with publish permissions
2. **Local Release**: Run `cargo publish -p <crate-name>` for each crate
3. **Git Tags**: Create and push git tags manually: `git tag <crate>-v<version>` and `git push --tags`

Note: Manual releases should only be needed for emergency fixes or if the automated system is unavailable.

### Configuration

Release automation is configured in:
- `.github/workflows/release-plz.yml` - GitHub Actions workflow
- `release-plz.toml` - release-plz configuration for workspace crates

### Required Secrets

For release-plz to publish crates, the repository must have the following secret configured in GitHub repository settings:
- `CARGO_REGISTRY_TOKEN` - A crates.io API token with publish permissions for the workspace crates

Maintainers with appropriate access can configure this in the repository settings under Secrets and variables → Actions.
