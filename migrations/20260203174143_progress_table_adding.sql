-- Add migration script here

CREATE TABLE daily_progress (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    day DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL,

    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE,

    CONSTRAINT unique_user_day
        UNIQUE (user_id, day)
);

CREATE TABLE daily_progress_todos (
    id UUID PRIMARY KEY,
    todo_id UUID NOT NULL,
    daily_progress_id UUID NOT NULL,

    is_done BOOLEAN NOT NULL DEFAULT false,

    created_at TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT fk_todo 
        FOREIGN KEY (todo_id)
        REFERENCES todos(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_daily_progress_fk
        FOREIGN KEY (daily_progress_id)
        REFERENCES daily_progress(id)
        ON DELETE CASCADE,

    CONSTRAINT unique_todo_per_day 
        UNIQUE (todo_id, daily_progress_id)
)