use async_graphql::{InputObject, OneofObject, SimpleObject};
use uuid::Uuid;

#[derive(SimpleObject, Debug)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: String,
}

#[derive(InputObject, Debug)]
pub struct NewIngredient {
    pub name: String,
}

#[derive(OneofObject, Debug)]
pub enum IngredientSource {
    /// A new ingredient.
    New(NewIngredient),
    /// The ID of an existing ingredient.
    Existing(Uuid),
}
