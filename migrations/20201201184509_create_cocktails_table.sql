CREATE TABLE cocktails (
    id UUID NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL,
    author_id UUID NOT NULL REFERENCES users,
    source TEXT,
    date_added TIMESTAMPTZ NOT NULL
);
