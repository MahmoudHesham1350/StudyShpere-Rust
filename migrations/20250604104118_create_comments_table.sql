CREATE TABLE comments (
    id SERIAL,
    material_id UUID NOT NULL REFERENCES materials(id) ON DELETE CASCADE,
    PRIMARY KEY(material_id, id),
    user_id UUID NULL REFERENCES users(id) ON DELETE SET NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
