use rusqlite::{Connection, NO_PARAMS};
use serde::{Serialize, Deserialize};
use serde_rusqlite::{from_row_with_columns, columns_from_statement};

type StdErr = Box<dyn std::error::Error>;

// Types prepared for being deserialized to by serde_json. Option values can be
// null. Continue doing this when you continue programming. :)

#[derive(Serialize, Deserialize, Debug)]
pub struct Cocktail {
    id: i64,
    name: String,
    date_added: i64,
    source: Option<String>,

    #[serde(skip)]
    ingredients: Vec<CocktailIngredient>,
}

#[derive(Serialize, Deserialize, Debug)]
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

    fn retrieve_all_cocktails(&self) -> Result<Vec<Cocktail>, StdErr> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, date_added, source FROM cocktails"
        )?;
        let cols = columns_from_statement(&stmt);
        let cocktails = stmt.query_and_then(NO_PARAMS, |row| from_row_with_columns::<Cocktail>(row, &cols))
            .unwrap()
            .filter_map(Result::ok)
            .collect();

        Ok(cocktails)
    }

    fn add_ingredients_to_cocktail_mut(&self, cocktail: &mut Cocktail) -> Result<(), StdErr> {
        let mut ing_stmt = self.conn.prepare(
            "SELECT
                ingredient_id AS id, label, amount, unit, ingredient_type
            FROM ingredients WHERE
                cocktail_id = ?"
        )?;
        let ing_cols = columns_from_statement(&ing_stmt);

        ing_stmt.query_and_then(&[cocktail.id], |row| from_row_with_columns::<CocktailIngredient>(row, &ing_cols))
            .unwrap()
            .filter_map(Result::ok)
            .for_each(|ing| cocktail.ingredients.push(ing));

        Ok(())
    }

    pub fn get_all_cocktails(&self) -> Result<Vec<Cocktail>, StdErr> {
        let mut cocktails = self.retrieve_all_cocktails()?;

        for cocktail in &mut cocktails {
            self.add_ingredients_to_cocktail_mut(cocktail)?;
        }

        Ok(cocktails)
    }
}
