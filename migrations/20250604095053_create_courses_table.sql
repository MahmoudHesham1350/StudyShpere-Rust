CREATE TABLE courses (
    group_name VARCHAR(255) NOT NULL REFERENCES groups(name),
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY (group_name, name),
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
