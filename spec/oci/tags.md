# OCI Tags

## Tag Upsert

r[oci.tag.upsert]
Upserting an OCI tag MUST be idempotent.

## Tag Classification

r[oci.tags.classify-release]
Release tags MUST be classified correctly.

r[oci.tags.classify-signature]
Signature tags MUST be classified correctly.

r[oci.tags.classify-attestation]
Attestation tags MUST be classified correctly.

r[oci.tags.classify-mixed]
Mixed tag lists MUST be classified correctly.

r[oci.tags.classify-empty]
Empty tag lists MUST be classified correctly.

r[oci.tags.classify-all-release]
Tag lists consisting entirely of release tags MUST be classified correctly.
