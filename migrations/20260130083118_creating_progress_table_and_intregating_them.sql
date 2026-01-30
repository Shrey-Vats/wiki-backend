-- Add migration script here
ALTER TABLE categories ADD CONSTRAINT unique_user_slug UNIQUE (user_id, slug);
ALTER TABLE tags ADD CONSTRAINT unique_user_slug UNIQUE (user_id, slug)
ALTER TABLE tag_todo ADD CONSTRAINT fk_todo FOREIGN KEY (todo_id) REFERENCES todos(id)

ALTER TABLE todos
ADD CONSTRAINT fk_category_todo 
FOREIGN KEY (category_id)
REFERENCES categories(id)
ON DELETE CASCADE;