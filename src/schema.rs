use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use juniper::FieldResult;

use crate::database;

#[derive(juniper::GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct Cocktail {
    pub id: i32,
    pub name: String,
    pub date_added: i32,
    pub source: Option<String>,
    pub author: String,

    #[serde(skip)]
    pub ingredients: Vec<CocktailIngredient>,
    #[serde(skip)]
    pub instructions: Vec<String>,
    #[serde(skip)]
    pub ratings: Vec<Rating>,
}

#[derive(juniper::GraphQLInputObject, Serialize, Deserialize, Debug)]
pub struct NewCocktail {
    name: String,
    source: Option<String>,
    author: String,
    ingredients: Vec<CocktailIngredientInput>,
    instructions: Vec<String>,
    ratings: Vec<NewRating>,
}

impl NewCocktail {
    pub fn to_cocktail(&self, id: i32, date_added: i32) -> Cocktail {
        Cocktail {
            id,
            date_added,
            name: self.name.clone(),
            source: self.source.clone(),
            author: self.author.clone(),
            ingredients: self.ingredients.clone().into_iter().map(|ing| ing.into()).collect(),
            instructions: self.instructions.clone(),
            ratings: self.ratings.clone().into_iter().map(|r| r.into()).collect(),
        }
    }
}

#[derive(juniper::GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct CocktailIngredient {
    label: String,
    amount: Option<f64>,
    unit: Option<String>,
    ingredient_type: Option<String>,
}

#[derive(juniper::GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
pub struct CocktailIngredientInput {
    label: String,
    amount: Option<f64>,
    unit: Option<String>,
    ingredient_type: Option<String>,
}

impl From<CocktailIngredientInput> for CocktailIngredient {
    fn from(ing: CocktailIngredientInput) -> CocktailIngredient {
        CocktailIngredient {
            label: ing.label.clone(),
            amount: ing.amount,
            unit: ing.unit.clone(),
            ingredient_type: ing.ingredient_type.clone(),
        }
    }
}

#[derive(juniper::GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct Rating {
    pub rating: i32,
    pub author: String,
}

#[derive(juniper::GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
pub struct NewRating {
    rating: i32,
    author: String,
}

impl From<NewRating> for Rating {
    fn from(rating: NewRating) -> Rating {
        Rating {
            rating: rating.rating,
            author: rating.author.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Context {
    pub db: Mutex<database::Database>,
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn apiVersion() -> &str {
        "0.1"
    }

    fn cocktail(ctx: &Context, id: i32) -> FieldResult<Cocktail> {
        let db = ctx.db.lock()?;
        let cocktail = db.get_cocktail(id)?;

        Ok(cocktail)
    }

    fn cocktails(context: &Context) -> FieldResult<Vec<Cocktail>> {
        let db = context.db.lock()?;
        let mut cocktails = db.get_all_cocktails()?;

        Ok(cocktails)
    }

    fn users(context: &Context) -> FieldResult<Vec<String>> {
        let db = context.db.lock()?;
        let users = db.get_all_users()?;

        Ok(users)
    }
}

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    fn createCocktail(context: &Context, new_cocktail: NewCocktail) -> FieldResult<Cocktail> {
        let db = context.db.lock()?;
        let cocktail = db.add_cocktail(&new_cocktail)?;

        Ok(cocktail)
    }

    fn editCocktail(context: &Context, id: i32, new_cocktail: NewCocktail) -> FieldResult<Cocktail> {
        let db = context.db.lock()?;
        let cocktail = db.overwrite_cocktail(id, &new_cocktail)?;

        Ok(cocktail)
    }

    fn deleteCocktail(context: &Context, id: i32) -> FieldResult<i32> {
        let db = context.db.lock()?;
        db.delete_cocktail(id)?;

        Ok(id)
    }

    fn rateCocktail(context: &Context, id: i32, rating: NewRating) -> FieldResult<Vec<Rating>> {
        let db = context.db.lock()?;
        db.rate_cocktail(id, rating.into())?;

        let ratings = db.get_cocktail(id)?.ratings;
        Ok(ratings)
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}

