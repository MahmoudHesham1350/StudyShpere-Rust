CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    bio TEXT NULL,
    image_url TEXT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
