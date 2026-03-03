# OCI Referrers

r[oci.referrer.insert]
OCI referrers MUST be insertable and listable.

r[oci.referrer.idempotent]
Referrer insertion MUST be idempotent.

r[oci.referrer.cascade-delete]
Deleting a manifest MUST cascade to its referrer relationships.
