# Init Command

The `init` subcommand scaffolds a new project directory.

r[init.current-dir]
Running `wasm init` without arguments MUST create the directory structure,
manifest, and lockfile in the current directory.

r[init.explicit-path]
Running `wasm init <path>` MUST create the directory structure and files at
the specified path.

r[init.composition-dirs]
Running `wasm init` MUST create the composition workspace directories:
`types/`, `seams/`, and `build/`.
