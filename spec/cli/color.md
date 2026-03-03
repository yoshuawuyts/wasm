# Color Support

r[cli.color.auto]
The CLI MUST accept `--color auto`.

r[cli.color.always]
The CLI MUST accept `--color always`.

r[cli.color.never]
The CLI MUST accept `--color never`.

r[cli.color.invalid]
The CLI MUST reject invalid `--color` values with an error message.

r[cli.color.in-help]
The `--color` flag MUST appear in `--help` output.

r[cli.color.no-color-env]
The CLI MUST respect the `NO_COLOR` environment variable.

r[cli.color.clicolor-env]
The CLI MUST respect the `CLICOLOR` environment variable.

r[cli.color.subcommand]
The `--color` flag MUST work when combined with subcommands.
