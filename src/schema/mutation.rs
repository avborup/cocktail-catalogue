use crate::schema::Context;

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {
}
