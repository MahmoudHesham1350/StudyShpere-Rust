CREATE TABLE material_labels (
    material_id UUID NOT NULL REFERENCES materials(id) ON DELETE CASCADE,
    group_name VARCHAR(255) NOT NULL,
    label_name VARCHAR(255) NOT NULL,
    number INTEGER NOT NULL,
    FOREIGN KEY (group_name, label_name) REFERENCES group_labels(group_name, name),
    PRIMARY KEY (material_id, group_name, label_name)
);
