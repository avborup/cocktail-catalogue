mod helpers;

use chrono::{DateTime, FixedOffset, Utc};
use helpers::{graphql_request, insert_user_in_db, spawn_app};
use serde_json::json;
use uuid::Uuid;

#[actix_rt::test]
async fn create_cocktail_returns_expected() {
    let app = spawn_app().await;

    // Prepare user
    insert_user_in_db(
        "Luke Skywalker",
        "d54090a3-0886-45b1-a8a7-2ec46c0938fe",
        &app.db_pool,
    )
    .await;

    // Prepare ingredient types
    let res_amaretto = graphql_request(
        &app.address,
        r#"
          mutation {
            createIngredientType(newIngredientType: { label: "Amaretto" }) {
              id
              label
            }
          }
        "#,
    )
    .await;
    let amaretto_json: serde_json::Value = res_amaretto
        .json()
        .await
        .expect("failed to read body of response");
    let amaretto_id = amaretto_json
        .pointer("/data/createIngredientType/id")
        .unwrap()
        .as_str()
        .unwrap();

    let res_bourbon = graphql_request(
        &app.address,
        r#"
          mutation {
            createIngredientType(newIngredientType: { label: "Bourbon" }) {
              id
              label
            }
          }
        "#,
    )
    .await;
    let bourbon_json: serde_json::Value = res_bourbon
        .json()
        .await
        .expect("failed to read body of response");
    let bourbon_id = bourbon_json
        .pointer("/data/createIngredientType/id")
        .unwrap()
        .as_str()
        .unwrap();

    // Main request
    let res = graphql_request(
        &app.address,
        r#"
          mutation {
            createCocktail(newCocktail: {
             name: "Amaretto Sour"
             authorId: "d54090a3-0886-45b1-a8a7-2ec46c0938fe"
             source: "https://jeffreymorgenthaler.com/i-make-the-best-amaretto-sour-in-the-world/"
             ingredients: [
               {
                 label: "Amaretto"
                 amount: 1.5
                 unit: "oz"
                 ingredientTypeId: "$amaretto_id"
               }
               {
                 label: "Cask-proof bourbon"
                 amount: 0.75
                 unit: "oz"
                 ingredientTypeId: "$bourbon_id"
               }
             ]
            }) {
             name
             author {
               id
               name
             }
             source
             ingredients {
               label
               amount
               unit
               ingredientType {
                 id
                 label
               }
             }
            }
          }
        "#
        .replace("$amaretto_id", amaretto_id)
        .replace("$bourbon_id", bourbon_id)
        .as_ref(),
    )
    .await;

    assert!(res.status().is_success());

    let expected = json!({
        "data": {
            "createCocktail": {
                "name": "Amaretto Sour",
                "author": {
                    "id": "d54090a3-0886-45b1-a8a7-2ec46c0938fe",
                    "name": "Luke Skywalker"
                },
                "source": "https://jeffreymorgenthaler.com/i-make-the-best-amaretto-sour-in-the-world/",
                "ingredients": [
                    {
                        "label": "Amaretto",
                        "amount": 1.5,
                        "unit": "oz",
                        "ingredientType": {
                            "id": amaretto_id,
                            "label": "Amaretto"
                        }
                    },
                    {
                        "label": "Cask-proof bourbon",
                        "amount": 0.75,
                        "unit": "oz",
                        "ingredientType": {
                            "id": bourbon_id,
                            "label": "Bourbon"
                        }
                    }
                ]
            }
        }
    });
    let actual: serde_json::Value = res.json().await.expect("failed to read body of response");

    assert_eq!(expected, actual);
}

#[actix_rt::test]
async fn create_cocktail_returns_id_and_date_added() {
    let app = spawn_app().await;

    insert_user_in_db(
        "Luke Skywalker",
        "d54090a3-0886-45b1-a8a7-2ec46c0938fe",
        &app.db_pool,
    )
    .await;

    let res = graphql_request(
        &app.address,
        r#"
          mutation {
            createCocktail(newCocktail: {
              name: "Amaretto Sour"
              authorId: "d54090a3-0886-45b1-a8a7-2ec46c0938fe"
              source: "https://jeffreymorgenthaler.com/i-make-the-best-amaretto-sour-in-the-world/"
              ingredients: []
            }) {
              id
              dateAdded
            }
          }
        "#,
    )
    .await;

    assert!(res.status().is_success());

    let json: serde_json::Value = res.json().await.expect("failed to read body of response");
    let id_option = json.pointer("/data/createCocktail/id");
    let date_added_option = json.pointer("/data/createCocktail/dateAdded");

    assert!(id_option.is_some());
    assert!(date_added_option.is_some());

    let id = id_option.unwrap();
    let date_added = date_added_option.unwrap();

    assert!(id.is_string());
    assert!(date_added.is_string());

    let id_parse_result = Uuid::parse_str(&id.as_str().unwrap());
    let date_added_parse_result =
        DateTime::<FixedOffset>::parse_from_rfc3339(&date_added.as_str().unwrap());

    assert!(id_parse_result.is_ok());
    assert!(date_added_parse_result.is_ok());
}

#[actix_rt::test]
async fn create_cocktail_saves_in_db() {
    let app = spawn_app().await;

    insert_user_in_db(
        "Luke Skywalker",
        "d54090a3-0886-45b1-a8a7-2ec46c0938fe",
        &app.db_pool,
    )
    .await;

    let res = graphql_request(
        &app.address,
        r#"
          mutation {
            createCocktail(newCocktail: {
              name: "Amaretto Sour"
              authorId: "d54090a3-0886-45b1-a8a7-2ec46c0938fe"
              source: "https://jeffreymorgenthaler.com/i-make-the-best-amaretto-sour-in-the-world/"
              ingredients: []
            }) {
              id
              dateAdded
            }
          }
        "#,
    )
    .await;

    assert!(res.status().is_success());

    let json: serde_json::Value = res.json().await.expect("failed to read body of response");
    let id = Uuid::parse_str(
        json.pointer("/data/createCocktail/id")
            .unwrap()
            .as_str()
            .unwrap(),
    )
    .unwrap();
    let date_added: DateTime<Utc> = DateTime::from_utc(
        DateTime::<FixedOffset>::parse_from_rfc3339(
            json.pointer("/data/createCocktail/dateAdded")
                .unwrap()
                .as_str()
                .unwrap(),
        )
        .unwrap()
        .naive_utc(),
        Utc,
    );

    let cocktail = sqlx::query!(
        "SELECT name, author_id, source, date_added FROM cocktails WHERE id = $1",
        id,
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("failed to get cocktail from db");

    assert_eq!(cocktail.name, "Amaretto Sour");
    assert_eq!(
        cocktail.author_id.to_string(),
        "d54090a3-0886-45b1-a8a7-2ec46c0938fe"
    );
    assert_eq!(
        cocktail.source.as_deref(),
        Some("https://jeffreymorgenthaler.com/i-make-the-best-amaretto-sour-in-the-world/")
    );
    assert_eq!(cocktail.date_added, date_added);
}

#[actix_rt::test]
async fn create_ingredient_type_returns_id_and_label() {
    let app = spawn_app().await;

    let res = graphql_request(
        &app.address,
        r#"
          mutation {
            createIngredientType(newIngredientType: { label: "Amaretto" }) {
              id
              label
            }
          }
        "#,
    )
    .await;

    assert!(res.status().is_success());

    let json: serde_json::Value = res.json().await.expect("failed to read body of response");
    let id_parse_result = Uuid::parse_str(
        json.pointer("/data/createIngredientType/id")
            .unwrap()
            .as_str()
            .unwrap(),
    );
    let label = json
        .pointer("/data/createIngredientType/label")
        .unwrap()
        .as_str()
        .unwrap();

    assert!(id_parse_result.is_ok());
    assert_eq!(label, "Amaretto");
}

#[actix_rt::test]
async fn create_ingredient_type_errors_on_duplicate() {
    let app = spawn_app().await;
    let query = r#"
      mutation {
        createIngredientType(newIngredientType: { label: "Amaretto" }) {
          id
          label
        }
      }
    "#;

    let res_first = graphql_request(&app.address, query).await;
    let res_second = graphql_request(&app.address, query).await;

    assert!(res_first.status().is_success());
    assert!(res_second.status().is_success());

    let json_first: serde_json::Value = res_first
        .json()
        .await
        .expect("failed to read body of response");
    let json_second: serde_json::Value = res_second
        .json()
        .await
        .expect("failed to read body of response");

    assert!(json_first["data"].is_object());
    assert!(json_first["errors"].is_null());
    assert!(json_second["data"].is_null());
    assert!(json_second["errors"].is_array());
}
