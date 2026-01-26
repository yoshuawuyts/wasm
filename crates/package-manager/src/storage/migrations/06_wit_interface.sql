-- WIT Interface storage tables

-- Store extracted WIT interface text for each component
CREATE TABLE IF NOT EXISTS wit_interface (
    id INTEGER PRIMARY KEY,
    -- The WIT text representation (full WIT document)
    wit_text TEXT NOT NULL,
    -- Parsed world name if available
    world_name TEXT,
    -- Number of imports in the interface
    import_count INTEGER NOT NULL DEFAULT 0,
    -- Number of exports in the interface  
    export_count INTEGER NOT NULL DEFAULT 0,
    -- Timestamp when this was extracted
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Link table between images and their WIT interfaces
-- An image can have one WIT interface
-- A WIT interface can be shared by multiple images (content-addressable)
CREATE TABLE IF NOT EXISTS image_wit_interface (
    image_id INTEGER NOT NULL,
    wit_interface_id INTEGER NOT NULL,
    PRIMARY KEY (image_id, wit_interface_id),
    FOREIGN KEY (image_id) REFERENCES image(id) ON DELETE CASCADE,
    FOREIGN KEY (wit_interface_id) REFERENCES wit_interface(id) ON DELETE CASCADE
);

-- Index for looking up interfaces by image
CREATE INDEX IF NOT EXISTS idx_image_wit_interface_image_id ON image_wit_interface(image_id);

-- Index for looking up images by interface
CREATE INDEX IF NOT EXISTS idx_image_wit_interface_wit_interface_id ON image_wit_interface(wit_interface_id);
