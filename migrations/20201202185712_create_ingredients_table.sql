CREATE TABLE ingredients (
    cocktail_id UUID NOT NULL REFERENCES cocktails,
    label TEXT NOT NULL,
    amount DOUBLE PRECISION,
    unit CHAR(2) CHECK (unit IN ('oz', 'ml')),
    ingredient_type_id UUID REFERENCES ingredient_types
);
