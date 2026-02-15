CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE  channels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE, 
    description TEXT,
    image_url TEXT,

    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT fk_channel_owner
        FOREIGN KEY (owner_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);

CREATE TABLE chat_rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    channel_id UUID NOT NULL,
    name TEXT NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL,

    CONSTRAINT fk_chat_room_channel
        FOREIGN KEY (channel_id)
        REFERENCES channels(id)
        ON DELETE CASCADE
);

CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chat_room_id UUID NOT NULL,
    sender_id UUID NOT NULL,
    content TEXT NOT NULL,
    is_edited BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT fk_message_room
        FOREIGN KEY (chat_room_id)
        REFERENCES chat_rooms(id)
        ON DELETE CASCADE,
    
    CONSTRAINT fk_message_sender 
        FOREIGN KEY (sender_id)
        REFERENCES users(id)
);

CREATE TABLE channel_members (
    user_id UUID NOT NULL,
    channel_id UUID NOT NULL ,

    joined_at TIMESTAMP NOT NULL DEFAULT now(),

    PRIMARY KEY (user_id, channel_id),

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE
);


CREATE INDEX idx_messages_room_created ON messages (chat_room_id, created_at);

CREATE INDEX idx_messages_sender ON messages (sender_id);