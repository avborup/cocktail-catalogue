use serde_json::json;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::helpers::{spawn_app, test_data::DAIQUIRI};

const CREATE_COCKTAIL_QUERY: &str = r#"
    mutation CreateCocktail($name: String!, $ingredients: [NewIngredient!]!) {
      createCocktail(newCocktail: { name: $name, ingredients: $ingredients }) {
        id
        name
      }
    }
"#;

#[sqlx::test]
async fn create_cocktail_returns_cocktail_fields(db: SqlitePool) {
    let app = spawn_app(db);

    let res = app.query_with_vars(CREATE_COCKTAIL_QUERY, DAIQUIRI).await;

    res.assert_no_errors();

    let id = res.get_string("/data/createCocktail/id");
    assert!(
        Uuid::parse_str(id).is_ok(),
        "Expected a valid UUID but got {id}"
    );

    assert_eq!("Daiquiri", res.get_string("/data/createCocktail/name"));
}

#[sqlx::test]
async fn create_cocktail_fails_on_empty_name(db: SqlitePool) {
    let app = spawn_app(db);

    let res = app
        .query_with_vars(
            CREATE_COCKTAIL_QUERY,
            &json!({ "name": "", "ingredients": [] }),
        )
        .await;

    let errors = res.get("/errors").as_array().unwrap();
    assert!(errors.len() == 1, "Expected one error but got {errors:?}");
    assert!(
        res.get_string("/errors/0/message")
            .contains("greater than or equal to 1"),
        "Expected error about empty name, got {errors:?}"
    );
}

const GET_COCKTAIL_QUERY: &str = r#"
    query GetCocktail($id: UUID!) {
      cocktail(id: $id) {
        id
        name
        ingredients {
          id
          label
          amount
          unit
        }
      }
    }
"#;

#[sqlx::test]
async fn query_cocktail_returns_cocktail_fields(db: SqlitePool) {
    let app = spawn_app(db);

    let create_res = app.query_with_vars(CREATE_COCKTAIL_QUERY, DAIQUIRI).await;
    let id = create_res.get_string("/data/createCocktail/id");

    let res = app
        .query_with_vars(GET_COCKTAIL_QUERY, &json!({ "id": id }))
        .await;

    res.assert_no_errors();

    assert_eq!(id, res.get_string("/data/cocktail/id"));
    assert_eq!("Daiquiri", res.get_string("/data/cocktail/name"));

    let mut ingredients = res.get_array("/data/cocktail/ingredients").clone();
    ingredients.sort_by(|a, b| {
        a["label"]
            .as_str()
            .unwrap()
            .cmp(b["label"].as_str().unwrap())
    });

    assert_eq!(3, ingredients.len());

    assert_eq!("Lime juice", ingredients[0]["label"]);
    assert_eq!(0.75, ingredients[0]["amount"].as_f64().unwrap());
    assert_eq!("oz", ingredients[0]["unit"]);

    assert_eq!("Simple syrup", ingredients[1]["label"]);
    assert_eq!(0.75, ingredients[1]["amount"].as_f64().unwrap());
    assert_eq!("oz", ingredients[1]["unit"]);

    assert_eq!("White rum", ingredients[2]["label"]);
    assert_eq!(2.0, ingredients[2]["amount"].as_f64().unwrap());
    assert_eq!("oz", ingredients[2]["unit"]);

    for ingredient in ingredients {
        let id = ingredient["id"].as_str().unwrap();
        assert!(
            Uuid::parse_str(id).is_ok(),
            "Expected a valid UUID but got {id}"
        );
    }
}
