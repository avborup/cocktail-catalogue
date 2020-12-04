mod helpers;

use helpers::{spawn_app, graphql_request};
use serde_json::json;

#[actix_rt::test]
async fn health_check_returns_200_empty() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let res = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("failed to execute request");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

#[actix_rt::test]
async fn api_version_returns_package_version() {
    let app = spawn_app().await;
    let res = graphql_request(
        &app.address,
        r#"
          {
            apiVersion
          }
        "#,
    )
    .await;

    assert!(res.status().is_success());

    let expected = json!({
        "data": {
            "apiVersion": env!("CARGO_PKG_VERSION")
        }
    });
    let actual: serde_json::Value = res.json().await.expect("failed to read body of response");

    assert_eq!(expected, actual);
}
