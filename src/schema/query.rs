use crate::schema::Context;

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn apiVersion() -> &str {
        env!("CARGO_PKG_VERSION")
    }
}
