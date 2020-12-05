use crate::schema::Context;
use juniper::graphql_object;

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn api_version() -> &str {
        env!("CARGO_PKG_VERSION")
    }
}
