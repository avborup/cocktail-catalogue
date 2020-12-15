use juniper::RootNode;

mod context;
mod mutation;
mod query;
mod subscription;
pub mod types;

pub use context::Context;
pub use mutation::Mutation;
pub use query::Query;
pub use subscription::Subscription;

pub type Schema = RootNode<'static, Query, Mutation, Subscription>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, Subscription)
}
