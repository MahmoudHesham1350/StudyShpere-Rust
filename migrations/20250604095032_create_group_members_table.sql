CREATE TABLE group_members (
    user_id UUID NOT NULL REFERENCES users(id),
    group_name VARCHAR(255) NOT NULL REFERENCES groups(name),
    user_role VARCHAR(50) DEFAULT 'member',
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    PRIMARY KEY (user_id, group_name)
);
