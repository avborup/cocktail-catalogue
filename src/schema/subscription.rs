use crate::schema::Context;
use juniper::graphql_object;

pub struct Subscription;

#[graphql_object(context = Context)]
impl Subscription {
    async fn unimplemented() -> Option<&str> {
        None
    }
}
