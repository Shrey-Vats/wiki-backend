-- Add migration script here
ALTER TABLE todos 
DROP CONSTRAINT todos_user_id_key,
DROP COLUMN is_done,
ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT now();

ALTER TABLE todos 
RENAME COLUMN "todo" TO title;