-- DROP TABLE IF EXISTS image;

CREATE TABLE IF NOT EXISTS image (
    id INTEGER PRIMARY KEY,
    ref_registry TEXT NOT NULL,
    ref_repository TEXT NOT NULL,
    ref_mirror_registry TEXT,
    ref_tag TEXT,
    ref_digest TEXT,
    manifest TEXT NOT NULL
);
