-- Add package_name column to wit_interface table
ALTER TABLE wit_interface ADD COLUMN package_name TEXT;

-- Create index for package_name lookups
CREATE INDEX IF NOT EXISTS idx_wit_interface_package_name ON wit_interface(package_name);
