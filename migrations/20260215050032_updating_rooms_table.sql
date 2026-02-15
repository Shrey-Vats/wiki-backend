-- Add migration script here
ALTER TABLE rooms ADD COLUMN slug TEXT UNIQUE NOT NULL;