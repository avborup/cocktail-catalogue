DROP TABLE cocktail_ingredients;
DROP TABLE ingredients;

CREATE TABLE ingredients (
  id TEXT PRIMARY KEY NOT NULL,
  cocktail_id TEXT NOT NULL,
  label TEXT NOT NULL,
  amount REAL,
  unit TEXT,

  FOREIGN KEY (cocktail_id) REFERENCES cocktails(id)
);
