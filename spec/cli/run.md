# Run Command

The `run` subcommand executes a WebAssembly component.

r[cli.run.help]
The CLI MUST provide `--help` output for the `run` command.

r[run.core-module-rejected]
The run command MUST reject core WebAssembly modules with a clear error message.

r[run.missing-file]
The run command MUST report a clear error when the target file does not exist.

r[run.oci-layer-lookup]
When running an OCI reference, the run command MUST retrieve the component
bytes using the `application/wasm` layer digest from the pulled manifest, not
the OCI reference string.
