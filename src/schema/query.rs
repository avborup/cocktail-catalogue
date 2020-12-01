use crate::schema::{Context, Cocktail};
use juniper::FieldResult;

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn apiVersion() -> &str {
        "0.1"
    }

    fn cocktail(ctx: &Context, id: i32) -> FieldResult<Cocktail> {
        let db = ctx.db.lock()?;
        let cocktail = db.get_cocktail(id)?;

        Ok(cocktail)
    }

    fn cocktails(context: &Context) -> FieldResult<Vec<Cocktail>> {
        let db = context.db.lock()?;
        let mut cocktails = db.get_all_cocktails()?;

        Ok(cocktails)
    }

    fn users(context: &Context) -> FieldResult<Vec<String>> {
        let db = context.db.lock()?;
        let users = db.get_all_users()?;

        Ok(users)
    }
}
