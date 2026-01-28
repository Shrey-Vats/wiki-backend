-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

ALTER TABLE todos
ALTER COLUMN id SET DEFAULT gen_random_uuid();
