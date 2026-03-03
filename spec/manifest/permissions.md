# Permissions

r[permissions.defaults]
Default permissions MUST resolve to correct values.

r[permissions.merge]
Permission merge MUST properly override fields from the base.

r[permissions.merge-preserve]
Permission merge MUST preserve base values when override is `None`.

r[permissions.serde]
Permissions MUST survive a serialization/deserialization roundtrip.

r[permissions.toml]
Permissions MUST be deserializable from TOML fragments.
