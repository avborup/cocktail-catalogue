use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(juniper::GraphQLInputObject, Debug)]
pub struct NewCocktail {
    pub name: String,
    pub author_id: Uuid,
    pub source: Option<String>,
}

#[derive(juniper::GraphQLObject, Debug)]
pub struct Cocktail {
    pub id: Uuid,
    pub name: String,
    pub author: User,
    pub source: Option<String>,
    pub date_added: DateTime<Utc>,
}

#[derive(juniper::GraphQLObject, Debug)]
pub struct User {
    pub name: String,
    pub id: Uuid,
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
