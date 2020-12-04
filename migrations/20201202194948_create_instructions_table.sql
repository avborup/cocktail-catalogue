CREATE TABLE instructions (
    cocktail_id UUID REFERENCES cocktails,
    step_number INTEGER NOT NULL,
    instruction TEXT NOT NULL
);
