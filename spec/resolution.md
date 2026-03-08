# Dependency Resolution

This document specifies the requirements for the PubGrub-based dependency
resolver.  Phase 1 covers the database, resolver core, and basic constraint
scenarios.  Phase 2 (version ranges, install integration) is tracked
separately.

## Per-Version Dependencies

r[resolution.per-version-deps]
The resolver MUST look up the dependencies of a package independently for each
(package, version) pair.  Different versions of the same package may declare
different sets of dependencies; the resolver MUST use the dependency set
recorded for the specific version being considered, not a union across all
versions.

## PubGrub Algorithm

r[resolution.pubgrub]
The package manager MUST use the PubGrub version-solving algorithm to resolve
the complete transitive dependency graph.  The algorithm MUST operate over all
(package, version) pairs present in the local database and MUST return a
conflict-free mapping from package name to the single selected version.

## Transitive Resolution

r[resolution.transitive]
The resolver MUST compute the complete transitive closure of dependencies.  If
package A depends on B and B depends on C, a resolution rooted at A MUST
include C in the output even when C is not a direct dependency of A.

## Diamond Dependencies

r[resolution.diamond]
The resolver MUST handle diamond dependency patterns.  When two packages in the
graph each declare a dependency on a third package that requires the same
version, the third package MUST appear exactly once in the resolved set at that
version.

## Conflict Detection

r[resolution.conflict-detection]
The resolver MUST detect incompatible version constraints.  When two packages
in the dependency graph require different, mutually-exclusive versions of the
same shared dependency, the resolver MUST return an error rather than
silently selecting one of the conflicting versions.
