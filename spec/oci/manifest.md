# OCI Manifest

r[oci.manifest.upsert]
Upserting an OCI manifest MUST store and retrieve correctly.

r[oci.manifest.annotations]
Manifest upsert MUST extract and store annotations.

r[oci.manifest.config-fields]
Manifest upsert MUST store config fields.

r[oci.manifest.placeholder-upgrade]
Upserting a manifest over a placeholder MUST upgrade it with full data.

r[oci.manifest.cascade-delete]
Deleting a manifest MUST cascade to layers, annotations, and referrers.
