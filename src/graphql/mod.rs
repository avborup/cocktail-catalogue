use async_graphql::{EmptySubscription, Schema};

pub(crate) use mutation::MutationRoot;
pub(crate) use query::QueryRoot;

mod cocktail;
mod mutation;
mod query;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) type ApiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
