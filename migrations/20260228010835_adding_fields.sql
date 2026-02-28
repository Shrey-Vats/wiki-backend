-- Add migration script here
ALTER TABLE daily_progress
ALTER COLUMN id SET DEFAULT gen_random_uuid();