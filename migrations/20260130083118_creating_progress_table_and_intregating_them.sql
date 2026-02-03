-- Add migration script here
ALTER TABLE categories ADD CONSTRAINT unique_user_slug UNIQUE (user_id, slug);
ALTER TABLE tag_todo ADD CONSTRAINT fk_todo FOREIGN KEY (todo_id) REFERENCES todos(id);
