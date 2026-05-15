CREATE TYPE notification_type AS ENUM ('menstion', 'reply', 'system');

CREATE TABLE notifications (
    id      UUID              PRIMARY KEY,
    user_id UUID              NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type    notification_type NOT NULL,
    title   TEXT NOT NULL,
    body    TEXT,
    created_at TIMESTAMP
)