CREATE TABLE jobs(
   job_id UUID PRIMARY KEY,   
   job_escrow_id TEXT NOT NULL UNIQUE,
   manifest_url TEXT,
   posted BIGINT
);
CREATE TABLE manifests(
   manifest_id UUID PRIMARY KEY,
   manifest_url TEXT,
   manifest_escrow_id TEXT UNIQUE,
   title TEXT,
   description TEXT,
   fortunes_required INTEGER
);
