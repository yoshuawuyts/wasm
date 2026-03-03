# Offline Mode

r[cli.offline.accepted]
The CLI MUST accept an `--offline` flag.

r[cli.offline.in-help]
The `--offline` flag MUST appear in `--help` output.

r[cli.offline.registry-blocked]
When `--offline` is set, registry operations MUST fail with a clear error
mentioning offline mode.

r[cli.offline.local-allowed]
When `--offline` is set, local operations MUST still succeed.

r[cli.offline.with-inspect]
The `--offline` flag MUST be accepted alongside the `registry inspect` command.

r[cli.offline.with-subcommand]
The `--offline` flag MUST be accepted alongside any subcommand.
