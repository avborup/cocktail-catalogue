use async_graphql::MergedObject;

use super::cocktail::CocktailMutation;

#[derive(MergedObject, Default)]
pub(crate) struct MutationRoot(CocktailMutation);
