use crate::database;
use crate::schema::types::{Cocktail, IngredientType, NewCocktail, NewIngredientType};
use crate::schema::Context;
use juniper::{graphql_object, FieldResult};

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn create_cocktail(
        context: &Context,
        new_cocktail: NewCocktail,
    ) -> FieldResult<Cocktail> {
        let cocktail = database::insert_new_cocktail(&context.db, new_cocktail).await?;
        Ok(cocktail)
    }

    async fn create_ingredient_type(
        context: &Context,
        new_ingredient_type: NewIngredientType,
    ) -> FieldResult<IngredientType> {
        let ingredient_type =
            database::insert_new_ingredient_type(&context.db, new_ingredient_type).await?;
        Ok(ingredient_type)
    }
}
