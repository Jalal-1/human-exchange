-- Create Jobs Table
CREATE TABLE jobs(
   job_id uuid NOT NULL,
   PRIMARY KEY (job_id),
   job_escrow_id TEXT NOT NULL UNIQUE,
   manifest_url TEXT,
   posted BIGINT
);