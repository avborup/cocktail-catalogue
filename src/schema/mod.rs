mod context;
mod mutation;
mod query;
mod types;

pub use context::Context;
pub use mutation::Mutation;
pub use query::Query;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
