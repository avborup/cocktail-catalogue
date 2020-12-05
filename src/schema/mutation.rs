use crate::schema::types::{
    Cocktail, Ingredient, IngredientType, NewCocktail, NewIngredientType, User,
};
use crate::schema::Context;
use chrono::Utc;
use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn createCocktail(context: &Context, new_cocktail: NewCocktail) -> FieldResult<Cocktail> {
        let author = sqlx::query_as!(
            User,
            "SELECT id, name FROM users WHERE id = $1",
            new_cocktail.author_id,
        )
        .fetch_one(&context.db)
        .await?;

        let id = Uuid::new_v4();
        let date_added = Utc::now();

        let db_output = sqlx::query!(
            "
            INSERT INTO cocktails (id, name, author_id, source, date_added)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, author_id, source, date_added
            ",
            id,
            new_cocktail.name,
            new_cocktail.author_id,
            new_cocktail.source,
            date_added,
        )
        .fetch_one(&context.db)
        .await?;

        let mut cocktail = Cocktail {
            id: db_output.id,
            name: db_output.name,
            author,
            source: db_output.source,
            date_added: db_output.date_added,
            ingredients: Vec::new(),
        };

        for new_ingredient in new_cocktail.ingredients {
            let ingredient_type = sqlx::query_as!(
                IngredientType,
                "SELECT id, label FROM ingredient_types WHERE id = $1",
                new_ingredient.ingredient_type_id,
            )
            .fetch_optional(&context.db)
            .await?;

            let db_output = sqlx::query!(
                "
                INSERT INTO ingredients (cocktail_id, label, amount, unit, ingredient_type_id)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING label, amount, unit
                ",
                id,
                new_ingredient.label,
                new_ingredient.amount,
                new_ingredient.unit,
                new_ingredient.ingredient_type_id
            )
            .fetch_one(&context.db)
            .await?;

            let ingredient = Ingredient {
                label: db_output.label,
                amount: db_output.amount,
                unit: db_output.unit,
                ingredient_type,
            };

            cocktail.ingredients.push(ingredient);
        }

        Ok(cocktail)
    }

    async fn createIngredientType(
        context: &Context,
        new_ingredient_type: NewIngredientType,
    ) -> FieldResult<IngredientType> {
        let id = Uuid::new_v4();
        let ingredient_type = sqlx::query_as!(
            IngredientType,
            "
            INSERT INTO ingredient_types (id, label)
            VALUES ($1, $2)
            RETURNING id, label
            ",
            id,
            new_ingredient_type.label,
        )
        .fetch_one(&context.db)
        .await?;

        Ok(ingredient_type)
    }
}
