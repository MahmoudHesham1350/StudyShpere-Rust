CREATE TABLE materials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_name VARCHAR(255) NOT NULL,
    course_name VARCHAR(255) NOT NULL,
    FOREIGN KEY (group_name, course_name) REFERENCES courses(group_name, name) ON DELETE CASCADE,
    
    title VARCHAR(255) NOT NULL,
    file VARCHAR(255),
    url VARCHAR(255),
    type VARCHAR(50) NOT NULL,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    creator UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE
    
);
