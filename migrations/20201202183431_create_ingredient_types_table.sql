CREATE TABLE ingredient_types (
    id UUID NOT NULL,
    label TEXT NOT NULL UNIQUE,
    PRIMARY KEY (id)
);
