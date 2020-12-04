CREATE TABLE ingredients (
    cocktail_id UUID REFERENCES cocktails,
    label TEXT NOT NULL,
    amount REAL,
    unit CHAR(2) CHECK (unit IN ('oz', 'ml')),
    ingredient_type_id INTEGER REFERENCES ingredient_types
);
