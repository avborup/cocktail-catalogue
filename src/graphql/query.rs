use async_graphql::{MergedObject, Object};

use super::{cocktail::CocktailQuery, VERSION};

#[derive(MergedObject, Default)]
pub struct QueryRoot(GeneralQuery, CocktailQuery);

#[derive(Default)]
pub struct GeneralQuery;

#[Object]
impl GeneralQuery {
    /// The version of the cocktail API.
    async fn api_version(&self) -> &str {
        VERSION
    }
}
