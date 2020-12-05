use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(juniper::GraphQLInputObject, Debug)]
pub struct NewCocktail {
    pub name: String,
    pub author_id: Uuid,
    pub source: Option<String>,
    pub ingredients: Vec<NewIngredient>,
}

#[derive(juniper::GraphQLObject, Debug)]
pub struct Cocktail {
    pub id: Uuid,
    pub name: String,
    pub author: User,
    pub source: Option<String>,
    pub date_added: DateTime<Utc>,
    pub ingredients: Vec<Ingredient>,
}

#[derive(juniper::GraphQLObject, Debug)]
pub struct User {
    pub name: String,
    pub id: Uuid,
}

#[derive(juniper::GraphQLInputObject, Debug)]
pub struct NewIngredient {
    pub label: String,
    pub amount: Option<f64>,
    pub unit: Option<String>,
    pub ingredient_type_id: Option<Uuid>,
}

#[derive(juniper::GraphQLObject, Debug)]
pub struct Ingredient {
    pub label: String,
    pub amount: Option<f64>,
    pub unit: Option<String>,
    pub ingredient_type: Option<IngredientType>,
}

#[derive(juniper::GraphQLInputObject, Debug)]
pub struct NewIngredientType {
    pub label: String,
}

#[derive(juniper::GraphQLObject, Debug)]
pub struct IngredientType {
    pub id: Uuid,
    pub label: String,
}
