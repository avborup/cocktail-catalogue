use crate::schema::types::{Cocktail, NewCocktail, User};
use crate::schema::Context;
use chrono::Utc;
use juniper::{FieldResult, graphql_object};
use uuid::Uuid;

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn createCocktail(_context: &Context, new_cocktail: NewCocktail) -> FieldResult<Cocktail> {
        let id = Uuid::new_v4();
        let date_added = Utc::now();

        // TODO: Add into database instead of returning the input
        let c = Cocktail {
            id,
            name: new_cocktail.name,
            author: User {
                name: "Adrian".to_string(),
                id: new_cocktail.author_id,
            },
            source: new_cocktail.source,
            date_added,
        };

        Ok(c)
    }
}
