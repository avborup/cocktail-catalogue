## SQL queries

 - Create recipe steps table
 - Create ratings table (add support for multiple people's ratings)

### Creating tables
Cocktails:
```SQL
sqlite> CREATE TABLE IF NOT EXISTS cocktails (
   ...>     id          INTEGER  PRIMARY KEY,
   ...>     name        TEXT     NOT NULL,
   ...>     source      TEXT,
   ...>     date_added  TEXT
   ...> );
```
Ingredients:
```SQL
sqlite> CREATE TABLE ingredients (
   ...>     ingredient_id   INTEGER PRIMARY KEY,
   ...>     cocktail_id     INTEGER NOT NULL,
   ...>     label           TEXT,
   ...>     amount          REAL,
   ...>     unit            TEXT    CHECK (unit IN ('oz', 'ml')),
   ...>     ingredient_type TEXT
   ...> );
```

### Creating cocktails table (Postgres)
```SQL
CREATE TABLE cocktails (
    id SERIAL PRIMARY KEY,
    name VARCHAR(75),
    source VARCHAR(250)
);
```

### Creating ingredients table (Postgres)
Change `name` to `label`.
```SQL
CREATE TABLE ingredients (
    ingredient_id SERIAL PRIMARY KEY,
    cocktail_id INT NOT NULL,
    label VARCHAR(75),
    amount DECIMAL,
    unit VARCHAR(25) check (unit in ('oz', 'ml')),
    ingredient_type VARCHAR(25)
);
```

## SQL structures
### Ingredients table example
| ingredient_id | cocktail_id |         name         | amount | unit |     type     |
|---------------|-------------|----------------------|--------|------|--------------|
|             5 |           1 | Lemon zest (garnish) |      1 |      |              |
|             4 |           1 | Egg white (optional) |     10 | ml   | Egg white    |
|             1 |           1 | Galliano             |     60 | ml   | Galliano     |
|             3 |           1 | Sugar syrup          |    7.5 | ml   | Simple syrup |
|             2 |           1 | Fresh lemon juice    |     30 | ml   | Lemon juice  |
|             7 |           2 | Blended Scotch whisky |      2 | oz   | Blended Scotch whisky |
|             9 |           2 | Honey-ginger syrup    |   0.75 | oz   | |
|             8 |           2 | Fresh lemon juice     |   0.75 | oz   | Lemon juice |
|            10 |           2 | Islay whisky          |   0.25 | oz   | Islay whisky |