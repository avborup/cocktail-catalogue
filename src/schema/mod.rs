mod mutation;
mod query;
mod context;
mod types;

pub use mutation::Mutation;
pub use query::Query;
pub use context::Context;
pub use types::{
    Cocktail,
    NewCocktail,
    CocktailIngredient,
    CocktailIngredientInput,
    Rating,
    NewRating,
};

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
