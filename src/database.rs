use rusqlite::{Connection, NO_PARAMS};

type StdErr = Box<dyn std::error::Error>;

// Types prepared for being deserialized to by serde_json. Option values can be
// null. Continue doing this when you continue programming. :)

#[derive(Debug)]
pub struct Cocktail {
    id: i64,
    name: String,
    date_added: i64,
    source: Option<String>,
    ingredients: Vec<CocktailIngredient>,
}

#[derive(Debug)]
pub struct CocktailIngredient {
    id: i64,
    label: String,
    amount: Option<f64>,
    unit: Option<String>,
    ingredient_type: Option<String>,
}

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: &str) -> Result<Self, StdErr> {
        let conn = Connection::open(path)?;
        Self::init_tables(&conn)?;

        Ok(Database { conn })
    }

    fn init_tables(conn: &Connection) -> rusqlite::Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cocktails (
                id          INTEGER  PRIMARY KEY,
                name        TEXT     NOT NULL,
                date_added  INT      NOT NULL,
                source      TEXT
            )",
            NO_PARAMS,
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS ingredients (
                ingredient_id   INTEGER PRIMARY KEY,
                cocktail_id     INTEGER NOT NULL,
                label           TEXT    NOT NULL,
                amount          REAL,
                unit            TEXT    CHECK (unit IN ('oz', 'ml')),
                ingredient_type TEXT
            )",
            NO_PARAMS,
        )?;
        
        Ok(())
    }

    pub fn get_all_cocktails(&self) -> Result<Vec<Cocktail>, StdErr> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, date_added, source FROM cocktails"
        )?;
        
        let cocktails_iter = stmt.query_map(
            NO_PARAMS,
            |row| {
                let mut cktl = Cocktail {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    date_added: row.get(2)?,
                    source: row.get(3)?,
                    ingredients: Vec::new(),
                };
                let cktl_id: i64 = row.get(0)?;

                let mut ing_stmt = self.conn.prepare(
                    "SELECT
                        ingredient_id, label, amount, unit, ingredient_type
                    FROM ingredients WHERE
                        cocktail_id = ?"
                )?;
                
                let ing_iter = ing_stmt.query_map(
                    &[cktl_id],
                    |ing_row| {
                        Ok(CocktailIngredient {
                            id: ing_row.get(0)?,
                            label: ing_row.get(1)?,
                            amount: ing_row.get(2)?,
                            unit: ing_row.get(3)?,
                            ingredient_type: ing_row.get(4)?,
                        })
                    }
                )?;

                for ing in ing_iter {
                    cktl.ingredients.push(ing?);
                }

                Ok(cktl)
            }
        )?;

        let mut cocktails = Vec::new();
        for c in cocktails_iter {
            cocktails.push(c?);
        }

        // let cocktails = cocktails_iter
        //     .filter_map(Result::ok)
        //     .collect();

        Ok(cocktails)
    }
}
