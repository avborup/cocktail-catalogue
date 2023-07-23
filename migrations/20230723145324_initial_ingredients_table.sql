CREATE TABLE IF NOT EXISTS ingredients (
  id TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cocktail_ingredients (
  cocktail_id TEXT NOT NULL,
  ingredient_id TEXT NOT NULL,

  FOREIGN KEY (cocktail_id) REFERENCES cocktails(id),
  FOREIGN KEY (ingredient_id) REFERENCES ingredients(id)
);
