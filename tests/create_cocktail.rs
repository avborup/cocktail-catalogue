mod helpers;

use chrono::{DateTime, FixedOffset};
use helpers::{graphql_request, spawn_app};
use serde_json::json;
use uuid::Uuid;

#[actix_rt::test]
async fn create_cocktail_returns_same() {
    let app = spawn_app().await;
    let res = graphql_request(
        &app.address,
        r#"
          mutation {
            createCocktail(newCocktail: {
              name: "Amaretto Sour"
              authorId: "d54090a3-0886-45b1-a8a7-2ec46c0938fe"
              source: "https://jeffreymorgenthaler.com/i-make-the-best-amaretto-sour-in-the-world/"
            }) {
              name
              author {
                id
                name
              }
              source
            }
          }
        "#,
    )
    .await;

    assert!(res.status().is_success());

    let expected = json!({
        "data": {
            "createCocktail": {
                "name": "Amaretto Sour",
                "author": {
                    "id": "d54090a3-0886-45b1-a8a7-2ec46c0938fe",
                    "name": "Adrian"
                },
                "source": "https://jeffreymorgenthaler.com/i-make-the-best-amaretto-sour-in-the-world/",
            }
        }
    });
    let actual: serde_json::Value = res.json().await.expect("failed to read body of response");

    assert_eq!(expected, actual);
}

#[actix_rt::test]
async fn create_cocktail_returns_id_and_date_added() {
    let app = spawn_app().await;
    let res = graphql_request(
        &app.address,
        r#"
          mutation {
            createCocktail(newCocktail: {
              name: "Amaretto Sour"
              authorId: "d54090a3-0886-45b1-a8a7-2ec46c0938fe"
              source: "https://jeffreymorgenthaler.com/i-make-the-best-amaretto-sour-in-the-world/"
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
    dbg!(&json);
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
