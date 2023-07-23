use async_graphql::{Context, InputObject, Object, SimpleObject};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(SimpleObject, Debug)]
pub struct Cocktail {
    pub id: Uuid,
    pub name: String,
}

#[derive(InputObject, Debug)]
pub struct NewCocktail {
    #[graphql(validator(min_length = 1))]
    pub name: String,
}

impl NewCocktail {
    pub fn into_cocktail(self, id: Uuid) -> Cocktail {
        Cocktail {
            id,
            name: self.name,
        }
    }
}

#[derive(Default)]
pub struct CocktailQuery;

#[Object]
impl CocktailQuery {
    /// Get a cocktail by ID.
    #[tracing::instrument(name = "Find cocktail", skip_all)]
    async fn cocktail(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Option<Cocktail>> {
        tracing::info!("Querying cocktail with ID: {id}");

        let db = ctx.data::<SqlitePool>()?;

        let cocktail = sqlx::query_as!(
            Cocktail,
            r#"
            SELECT id as "id: Uuid", name
            FROM cocktails
            WHERE id = ?1
            "#,
            id
        )
        .fetch_optional(db)
        .await?;

        Ok(cocktail)
    }
}

#[derive(Default)]
pub struct CocktailMutation;

#[Object]
impl CocktailMutation {
    /// Create a new cocktail.
    #[tracing::instrument(name = "Create cocktail", skip_all)]
    async fn create_cocktail(
        &self,
        ctx: &Context<'_>,
        new_cocktail: NewCocktail,
    ) -> async_graphql::Result<Cocktail> {
        tracing::info!("Creating cocktail with name: {}", new_cocktail.name);

        let db = ctx.data::<SqlitePool>()?;
        let cocktail_id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO cocktails (id, name)
            VALUES (?1, ?2)
            "#,
            cocktail_id,
            new_cocktail.name
        )
        .execute(db)
        .await?;

        tracing::info!("Created cocktail with ID: {cocktail_id}");

        Ok(new_cocktail.into_cocktail(cocktail_id))
    }
}
