# Dotenv

The CLI supports loading environment variables from `.env` files.

r[dotenv.detection]
The CLI MUST detect and report the presence of a `.env` file in `self config`
output, including the number of variables defined.

r[dotenv.not-found]
When no `.env` file exists, the CLI MUST report it as `not found`.

r[dotenv.loading]
The CLI MUST load variables from a `.env` file successfully.

r[dotenv.precedence]
System environment variables MUST take precedence over `.env` file variables.
