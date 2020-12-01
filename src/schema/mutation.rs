use crate::schema::{Context, Cocktail, NewCocktail, Rating, NewRating};
use juniper::FieldResult;

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    fn createCocktail(context: &Context, new_cocktail: NewCocktail) -> FieldResult<Cocktail> {
        let db = context.db.lock()?;
        let cocktail = db.add_cocktail(&new_cocktail)?;

        Ok(cocktail)
    }

    fn editCocktail(context: &Context, id: i32, new_cocktail: NewCocktail) -> FieldResult<Cocktail> {
        let db = context.db.lock()?;
        let cocktail = db.overwrite_cocktail(id, &new_cocktail)?;

        Ok(cocktail)
    }

    fn deleteCocktail(context: &Context, id: i32) -> FieldResult<i32> {
        let db = context.db.lock()?;
        db.delete_cocktail(id)?;

        Ok(id)
    }

    fn rateCocktail(context: &Context, id: i32, rating: NewRating) -> FieldResult<Vec<Rating>> {
        let db = context.db.lock()?;
        db.rate_cocktail(id, rating.into())?;

        let ratings = db.get_cocktail(id)?.ratings;
        Ok(ratings)
    }
}
