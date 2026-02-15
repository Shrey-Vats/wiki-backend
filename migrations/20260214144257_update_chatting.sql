
CREATE TABLE rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL,
    profile_pic TEXT,
    name TEXT NOT NULL,
    description TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT fk_rooms_owner
        FOREIGN KEY (owner_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);
CREATE INDEX idx_rooms_owner ON rooms(owner_id);

CREATE TABLE members (
    user_id UUID NOT NULL,
    room_id UUID NOT NULL,

    PRIMARY KEY(user_id, room_id),

    CONSTRAINT fk_members_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_members_room
        FOREIGN KEY (room_id)
        REFERENCES rooms(id)
        ON DELETE CASCADE
);

CREATE INDEX idx_members_room ON members(room_id);

CREATE TABLE user_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID, 
    room_id UUID NOT NULL,
    content TEXT NOT NULL,

    
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    CONSTRAINT fk_messages_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE SET NULL,

    CONSTRAINT fk_messages_room
        FOREIGN KEY (room_id)
        REFERENCES rooms(id)
        ON DELETE CASCADE
);
