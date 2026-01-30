-- Add migration script here

CREATE TABLE tag_todo (
    todo_id UUID NOT NULL,
    tag_id UUID NOT NULL,

    PRIMARY KEY (todo_id, tag_id)
);

ALTER TABLE todos ADD COLUMN category_id UUID NOT NULL;

ALTER TABLE todos
ADD CONSTRAINT fk_category_todo 
FOREIGN KEY (category_id)
REFERENCES categories(id)
ON DELETE CASCADE;