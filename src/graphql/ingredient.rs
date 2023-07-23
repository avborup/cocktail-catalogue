use async_graphql::{InputObject, SimpleObject};
use uuid::Uuid;

#[derive(SimpleObject, Debug)]
pub struct Ingredient {
    pub id: Uuid,
    pub label: String,
    pub amount: Option<f64>,
    pub unit: Option<String>,
}

#[derive(InputObject, Debug)]
pub struct NewIngredient {
    #[graphql(validator(min_length = 1))]
    pub label: String,
    pub amount: Option<f64>,
    pub unit: Option<String>,
}
