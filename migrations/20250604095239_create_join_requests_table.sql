CREATE TABLE join_requests (
    group_name VARCHAR(255) REFERENCES groups(name) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (group_name, user_id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
