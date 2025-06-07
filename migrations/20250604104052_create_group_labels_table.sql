CREATE TABLE group_labels (
    group_name VARCHAR(255) NOT NULL REFERENCES groups(name) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY (group_name, name)
);
