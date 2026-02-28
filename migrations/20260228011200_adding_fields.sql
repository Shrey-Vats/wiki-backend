-- Add migration script here
ALTER TABLE daily_progress
ALTER COLUMN updated_at SET DEFAULT now();