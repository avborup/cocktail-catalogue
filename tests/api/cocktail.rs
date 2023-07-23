use serde_json::json;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::helpers::spawn_app;

const CREATE_COCKTAIL_QUERY: &str = r#"
    mutation CreateCocktail($name: String!) {
      createCocktail(newCocktail: { name: $name }) {
        id
        name
      }
    }
"#;

#[sqlx::test]
async fn create_cocktail_returns_cocktail_fields(db: SqlitePool) {
    let app = spawn_app(db);

    let res = app
        .query_with_vars(CREATE_COCKTAIL_QUERY, json!({ "name": "Margarita" }))
        .await;

    res.assert_no_errors();

    let id = res.get_string("/data/createCocktail/id");
    assert!(
        Uuid::parse_str(id).is_ok(),
        "Expected a valid UUID but got {id}"
    );

    assert_eq!("Margarita", res.get_string("/data/createCocktail/name"));
}

#[sqlx::test]
async fn create_cocktail_fails_on_empty_name(db: SqlitePool) {
    let app = spawn_app(db);

    let res = app
        .query_with_vars(CREATE_COCKTAIL_QUERY, json!({ "name": "" }))
        .await;

    let errors = res.get("/errors").as_array().unwrap();
    assert!(errors.len() == 1, "Expected one error but got {errors:?}");
    assert!(res
        .get_string("/errors/0/message")
        .contains("greater than or equal to 1"));
}

const GET_COCKTAIL_QUERY: &str = r#"
    query GetCocktail($id: UUID!) {
      cocktail(id: $id) {
        id
        name
      }
    }
"#;

#[sqlx::test]
async fn query_cocktail_returns_cocktail_fields(db: SqlitePool) {
    let app = spawn_app(db);

    let create_res = app
        .query_with_vars(CREATE_COCKTAIL_QUERY, json!({ "name": "Daiquiri" }))
        .await;
    let id = create_res.get_string("/data/createCocktail/id");

    dbg!(&id);

    let res = app
        .query_with_vars(GET_COCKTAIL_QUERY, json!({ "id": id }))
        .await;

    res.assert_no_errors();

    assert_eq!(id, res.get_string("/data/cocktail/id"));
    assert_eq!("Daiquiri", res.get_string("/data/cocktail/name"));
}
