-- Add tag_type column to distinguish regular tags from signatures and attestations
ALTER TABLE known_package_tag ADD COLUMN tag_type TEXT NOT NULL DEFAULT 'release';

-- Update existing signature and attestation tags
UPDATE known_package_tag SET tag_type = 'signature' WHERE tag LIKE '%.sig';
UPDATE known_package_tag SET tag_type = 'attestation' WHERE tag LIKE '%.att';
