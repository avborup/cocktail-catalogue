use rusqlite::{Connection, NO_PARAMS, ToSql};
use serde::{Serialize, Deserialize};
use serde_rusqlite::{from_row_with_columns, columns_from_statement, to_params_named};
use crate::utils::get_cur_time_unix;

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
    #[serde(skip)]
    instructions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CocktailBasic {
    pub name: String,
    pub source: Option<String>,
    pub ingredients: Vec<CocktailIngredient>,
    pub instructions: Vec<String>,
}

impl CocktailBasic {
    pub fn to_cocktail(&self, id: i64, date_added: i64) -> Cocktail {
        Cocktail {
            id,
            date_added,
            name: self.name.clone(),
            source: self.source.clone(),
            ingredients: self.ingredients.clone(),
            instructions: self.instructions.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CocktailIngredient {
    pub label: String,
    pub amount: Option<f64>,
    pub unit: Option<String>,
    pub ingredient_type: Option<String>,
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
                cocktail_id     INTEGER NOT NULL,
                label           TEXT    NOT NULL,
                amount          REAL,
                unit            TEXT    CHECK (unit IN ('oz', 'ml')),
                ingredient_type TEXT
            )",
            NO_PARAMS,
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS instructions (
                cocktail_id     INTEGER NOT NULL,
                step_number     INTEGER NOT NULL,
                instruction     TEXT    NOT NULL
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

    fn add_instructions_to_cocktail_mut(&self, cocktail: &mut Cocktail) -> Result<(), StdErr> {
        let mut stmt = self.conn.prepare("SELECT instruction FROM instructions WHERE cocktail_id = ? ORDER BY step_number")?;
        
        stmt.query_and_then(&[cocktail.id], |row| row.get(0))?
            .filter_map(Result::ok)
            .for_each(|instr| cocktail.instructions.push(instr));

        Ok(())
    }

    pub fn get_all_cocktails(&self) -> Result<Vec<Cocktail>, StdErr> {
        let mut cocktails = self.retrieve_all_cocktails()?;

        for cocktail in &mut cocktails {
            self.add_ingredients_to_cocktail_mut(cocktail)?;
            self.add_instructions_to_cocktail_mut(cocktail)?;
        }

        Ok(cocktails)
    }

    pub fn generate_id(&self) -> Result<i64, StdErr> {
        let mut stmt = self.conn.prepare("SELECT id FROM cocktails ORDER BY id DESC LIMIT 1")?;
        let rows = stmt.query_map(NO_PARAMS, |row| row.get(0))?;

        for id_result in rows {
            let id: i64 = id_result?;
            return Ok(id + 1);
        }

        Err("Unable to generate id")?
    }

    pub fn add_cocktail(&self, cb: &CocktailBasic) -> Result<(), StdErr> {
        let id = self.generate_id()?;
        let date_added = get_cur_time_unix()?;
        let cocktail = cb.to_cocktail(id, date_added as i64);

        self.add_cocktail_to_db(&cocktail)?;

        Ok(())
    }
    
    pub fn add_cocktail_to_db(&self, cocktail: &Cocktail) -> Result<(), StdErr> {
        self.conn.execute_named("INSERT INTO cocktails (id, name, date_added, source)
                                VALUES (:id, :name, :date_added, :source)",
                                &to_params_named(&cocktail).unwrap().to_slice())?;

        self.add_ingredients_to_db(cocktail.id, &cocktail.ingredients)?;
        self.add_instructions_to_db(cocktail.id, &cocktail.instructions)?;

        Ok(())
    }

    fn add_ingredients_to_db(&self, cocktail_id: i64, ingredients: &[CocktailIngredient]) -> Result<(), StdErr> {
        for ingredient in ingredients {
            let p1 = to_params_named(&ingredient).unwrap();
            let p2: Vec<(&str, &dyn ToSql)> = vec![(":cocktail_id", &cocktail_id)];
            let params = [p1.to_slice().as_slice(), p2.as_slice()].concat();

            self.conn.execute_named("INSERT INTO ingredients (cocktail_id, label, amount, unit, ingredient_type)
                                    VALUES (:cocktail_id, :label, :amount, :unit, :ingredient_type)",
                                    &params)?;
        }

        Ok(())
    }

    fn add_instructions_to_db(&self, cocktail_id: i64, instructions: &[String]) -> Result<(), StdErr> {
        for i in 0..instructions.len() {
            let instruction = &instructions[i];
            let step_num = (i + 1) as i8;

            self.conn.execute_named("INSERT INTO instructions (cocktail_id, step_number, instruction)
                                    VALUES (:cocktail_id, :step_number, :instruction)",
                                    &[(":cocktail_id", &cocktail_id), (":step_number", &step_num), (":instruction", &instruction)])?;
        }

        Ok(())
    }

    pub fn delete_cocktail(&self, id: i64) -> Result<(), rusqlite::Error> {
        self.conn.execute_named("DELETE FROM ingredients WHERE cocktail_id = :cocktail_id", &[(":cocktail_id", &id)])?;
        self.conn.execute_named("DELETE FROM instructions WHERE cocktail_id = :cocktail_id", &[(":cocktail_id", &id)])?;
        self.conn.execute_named("DELETE FROM cocktails WHERE id = :cocktail_id", &[(":cocktail_id", &id)])?;

        Ok(())
    }
    
    pub fn overwrite_cocktail(&self, cocktail: &Cocktail) -> Result<(), StdErr> {
        self.delete_cocktail(cocktail.id)?;
        self.add_cocktail_to_db(&cocktail)?;

        Ok(())
    }
}
