use crate::schema::types::{
    Cocktail, Ingredient, IngredientType, NewCocktail, NewIngredient, NewIngredientType, User,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_new_cocktail(
    db: &PgPool,
    new_cocktail: NewCocktail,
) -> Result<Cocktail, sqlx::error::Error> {
    let author = get_user(db, new_cocktail.author_id).await?;

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
    .fetch_one(db)
    .await?;

    let ingredients = insert_new_ingredients(db, id, &new_cocktail.ingredients).await?;

    let cocktail = Cocktail {
        id: db_output.id,
        name: db_output.name,
        author,
        source: db_output.source,
        date_added: db_output.date_added,
        ingredients,
    };

    Ok(cocktail)
}

async fn get_user(db: &PgPool, user_id: Uuid) -> Result<User, sqlx::error::Error> {
    sqlx::query_as!(User, "SELECT id, name FROM users WHERE id = $1", user_id,)
        .fetch_one(db)
        .await
}

async fn insert_new_ingredients(
    db: &PgPool,
    cocktail_id: Uuid,
    new_ingredients: &[NewIngredient],
) -> Result<Vec<Ingredient>, sqlx::error::Error> {
    let mut ingredients = Vec::with_capacity(new_ingredients.len());

    for new_ingredient in new_ingredients {
        let ingredient_type = sqlx::query_as!(
            IngredientType,
            "SELECT id, label FROM ingredient_types WHERE id = $1",
            new_ingredient.ingredient_type_id,
        )
        .fetch_optional(db)
        .await?;

        let db_output = sqlx::query!(
            "
            INSERT INTO ingredients (cocktail_id, label, amount, unit, ingredient_type_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING label, amount, unit
            ",
            cocktail_id,
            new_ingredient.label,
            new_ingredient.amount,
            new_ingredient.unit,
            new_ingredient.ingredient_type_id
        )
        .fetch_one(db)
        .await?;

        let ingredient = Ingredient {
            label: db_output.label,
            amount: db_output.amount,
            unit: db_output.unit,
            ingredient_type,
        };

        ingredients.push(ingredient);
    }

    Ok(ingredients)
}

pub async fn insert_new_ingredient_type(
    db: &PgPool,
    new_ingredient_type: NewIngredientType,
) -> Result<IngredientType, sqlx::error::Error> {
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
    .fetch_one(db)
    .await?;

    Ok(ingredient_type)
}
