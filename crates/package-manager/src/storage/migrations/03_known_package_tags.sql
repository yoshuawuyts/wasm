-- Table for tags associated with known packages
CREATE TABLE IF NOT EXISTS known_package_tag (
    id INTEGER PRIMARY KEY,
    known_package_id INTEGER NOT NULL,
    tag TEXT NOT NULL,
    last_seen_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (known_package_id) REFERENCES known_package(id) ON DELETE CASCADE,
    UNIQUE(known_package_id, tag)
);

-- Index for faster tag lookups
CREATE INDEX IF NOT EXISTS idx_known_package_tag_package_id ON known_package_tag(known_package_id);
