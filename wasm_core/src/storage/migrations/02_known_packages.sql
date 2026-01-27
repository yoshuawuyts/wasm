-- Table for known packages that persists even after local deletion
-- This tracks packages the user has seen/searched for
CREATE TABLE IF NOT EXISTS known_package (
    id INTEGER PRIMARY KEY,
    registry TEXT NOT NULL,
    repository TEXT NOT NULL,
    description TEXT,
    last_seen_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    -- Ensure unique package per registry/repository combination
    UNIQUE(registry, repository)
);

-- Index for faster searches
CREATE INDEX IF NOT EXISTS idx_known_package_repository ON known_package(repository);
CREATE INDEX IF NOT EXISTS idx_known_package_registry ON known_package(registry);
