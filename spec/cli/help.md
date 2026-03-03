# Help

The `wasm` command-line interface provides subcommands for managing WebAssembly
components and interfaces.

r[cli.help]
The CLI MUST provide `--help` output for the top-level command.

## Offline Mode

r[cli.offline.accepted]
The CLI MUST accept an `--offline` flag.

r[cli.offline.in-help]
The `--offline` flag MUST appear in `--help` output.

r[cli.offline.with-subcommand]
The `--offline` flag MUST be accepted alongside any subcommand.
