-- Table for storing component interfaces derived from image manifests
-- This is a derived table that can be re-scanned after migrations
CREATE TABLE IF NOT EXISTS interface (
    id INTEGER PRIMARY KEY,
    image_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    interface_type TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (image_id) REFERENCES image(id) ON DELETE CASCADE
);

-- Index for faster lookups by image_id
CREATE INDEX IF NOT EXISTS idx_interface_image_id ON interface(image_id);

-- Index for faster searches by interface name
CREATE INDEX IF NOT EXISTS idx_interface_name ON interface(name);
