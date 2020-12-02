CREATE TABLE cocktails (
    id          UUID        NOT NULL,
    PRIMARY KEY (id),
    name        TEXT        NOT NULL,
    author      TEXT        NOT NULL,
    source      TEXT,
    date_added  TIMESTAMPTZ NOT NULL
);
