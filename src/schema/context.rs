use sqlx::PgPool;

#[derive(Debug)]
pub struct Context {
    pub db: PgPool,
}

impl juniper::Context for Context {}
