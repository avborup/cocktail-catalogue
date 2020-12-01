use crate::schema::Context;

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {}
