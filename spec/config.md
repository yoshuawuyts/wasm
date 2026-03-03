# Configuration

The `config` module manages global and local configuration files.

r[config.default]
A default configuration MUST be constructable.

r[config.load-missing]
Loading a nonexistent config file MUST succeed gracefully.

r[config.load-valid]
Loading a valid config file MUST return the correct settings.

r[config.ensure-exists]
`ensure_exists` MUST create the config file if it is missing.

r[config.ensure-idempotent]
`ensure_exists` MUST be idempotent.

r[config.credentials.cache]
Credential caching MUST work correctly.

r[config.credentials.no-helper]
Missing credential helpers MUST be handled gracefully.

r[config.local-overrides]
Local configuration MUST override global configuration.
