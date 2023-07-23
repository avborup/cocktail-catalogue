use async_graphql::{MergedObject, Object};

use super::VERSION;

#[derive(MergedObject, Default)]
pub struct QueryRoot(GeneralQuery);

#[derive(Default)]
pub struct GeneralQuery;

#[Object]
impl GeneralQuery {
    /// The version of the cocktail API.
    async fn api_version(&self) -> &str {
        VERSION
    }
}
