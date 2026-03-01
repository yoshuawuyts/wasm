# GitHub Action

The `wasm` CLI ships a reusable [GitHub Action][actions-docs] that you can use
to run the `wasm(1)` tool in your CI/CD workflows. This is useful for
automating tasks such as running WebAssembly components, installing
dependencies from OCI registries, or validating local Wasm files.

## Usage

Reference the action from the `yoshuawuyts/wasm-cli` repository in any
workflow step:

```yaml
- uses: yoshuawuyts/wasm-cli@main
  with:
    command: run
    input: path/to/component.wasm
```

## Inputs

| Input              | Required | Default  | Description                                                              |
| ------------------ | -------- | -------- | ------------------------------------------------------------------------ |
| `command`          | yes      | `run`    | The `wasm` subcommand to run (`run`, `install`, `init`, `local`, `registry`) |
| `input`            | no       | —        | For `wasm run`: local file path or OCI reference to the Wasm Component   |
| `args`             | no       | —        | Additional arguments passed verbatim to the `wasm` command               |
| `offline`          | no       | `false`  | Run in offline mode (`--offline`)                                        |
| `color`            | no       | `auto`   | When to use colored output: `auto`, `always`, or `never` (`--color`)    |
| `inherit-env`      | no       | `false`  | For `wasm run`: inherit all host environment variables (`--inherit-env`) |
| `inherit-network`  | no       | `false`  | For `wasm run`: allow the guest to access the network (`--inherit-network`) |
| `no-stdio`         | no       | `false`  | For `wasm run`: suppress stdin/stdout/stderr inheritance (`--no-stdio`)  |

## Outputs

| Output      | Description                              |
| ----------- | ---------------------------------------- |
| `exit-code` | Exit code returned by the `wasm` command |

## Examples

### Run a local Wasm Component

```yaml
steps:
  - uses: actions/checkout@v4
  - uses: yoshuawuyts/wasm-cli@main
    with:
      command: run
      input: my-component.wasm
```

### Install a component from an OCI registry

```yaml
steps:
  - uses: actions/checkout@v4
  - uses: yoshuawuyts/wasm-cli@main
    with:
      command: install
      input: ghcr.io/bytecodealliance/sample-wasi-http-rust/sample-wasi-http-rust:0.1.6
```

### Run a component with network access

```yaml
steps:
  - uses: actions/checkout@v4
  - uses: yoshuawuyts/wasm-cli@main
    with:
      command: run
      input: my-component.wasm
      inherit-network: 'true'
```

### Pass environment variables to the guest

```yaml
steps:
  - uses: actions/checkout@v4
  - uses: yoshuawuyts/wasm-cli@main
    with:
      command: run
      input: my-component.wasm
      args: '--env API_URL=https://example.com --env DEBUG=1'
```

### Check the exit code

```yaml
steps:
  - uses: actions/checkout@v4
  - name: Run component
    id: run
    uses: yoshuawuyts/wasm-cli@main
    with:
      command: run
      input: my-component.wasm
  - name: Handle result
    run: |
      echo "Exit code: ${{ steps.run.outputs.exit-code }}"
      if [ "${{ steps.run.outputs.exit-code }}" != "0" ]; then
        echo "Component failed"
        exit 1
      fi
```

### Run in offline mode

```yaml
steps:
  - uses: actions/checkout@v4
  - uses: yoshuawuyts/wasm-cli@main
    with:
      command: run
      input: my-component.wasm
      offline: 'true'
```

[actions-docs]: https://docs.github.com/en/actions
