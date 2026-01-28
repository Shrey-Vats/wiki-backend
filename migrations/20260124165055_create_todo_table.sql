-- Add migration script here
CREATE TABLE todos (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL UNIQUE,
    todo TEXT NOT NULL,
    description TEXT NOT NULL,
    is_done BOOLEAN DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
)