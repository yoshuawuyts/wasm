-- Add size_on_disk column to track the storage size of each image
ALTER TABLE image ADD COLUMN size_on_disk INTEGER NOT NULL DEFAULT 0;
