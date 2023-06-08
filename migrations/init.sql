-- Add migration script here
create TABLE IF NOT EXISTS jobs
(
    job_id   TEXT PRIMARY KEY NOT NULL,
    shortcode TEXT UNIQUE NOT NULL,
    escrow_id   TEXT NOT NULL,
    manifest_url TEXT,
    posted    BIGINT NOT NULL,
    expires   DATETIME,
    password  TEXT,
    responses      BIGINT NOT NULL
);
-- Add migration script here
create TABLE IF NOT EXISTS manifests
(
    manifest_id   TEXT PRIMARY KEY NOT NULL,
    manifest_escrow_id   TEXT NOT NULL,
    title TEXT,
    description  TEXT
);