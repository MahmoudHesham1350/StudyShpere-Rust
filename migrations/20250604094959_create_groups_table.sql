CREATE TABLE groups (
    owner_id UUID NOT NULL REFERENCES users(id),
    name VARCHAR(255) PRIMARY KEY,
    description TEXT NOT NULL,
    join_type VARCHAR(50) NOT NULL,
    post_permission VARCHAR(50) DEFAULT 'ADMIN',
    edit_permissions VARCHAR(50) DEFAULT 'ADMIN',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
