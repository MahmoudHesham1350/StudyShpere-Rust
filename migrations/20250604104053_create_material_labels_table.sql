CREATE TABLE material_labels (
    material_id UUID NOT NULL REFERENCES materials(id),
    label_id UUID NOT NULL, -- Assuming a 'labels' table will be created later
    number INTEGER NOT NULL,
    PRIMARY KEY (material_id, label_id)
);
