use std::ops::Deref;

use eyre::Context;
use serde_json::{json, to_string_pretty, Value};

use super::TestApp;

pub struct GraphQLClient {
    client: reqwest::Client,
    url: String,
}

impl Deref for TestApp {
    type Target = GraphQLClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl GraphQLClient {
    pub fn new(url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            url,
        }
    }

    async fn send_query(
        &self,
        query: impl AsRef<str>,
        variables: Option<Value>,
    ) -> GraphQLResponse {
        let body = json!({
            "query": query.as_ref(),
            "variables": variables.unwrap_or(Value::Null),
        });

        let res = self
            .client
            .post(&self.url)
            .json(&body)
            .send()
            .await
            .wrap_err("Failed to send query")
            .unwrap();

        let body = res
            .json()
            .await
            .wrap_err("Failed to parse response as JSON")
            .unwrap();

        GraphQLResponse { body }
    }

    pub async fn query_with_vars(
        &self,
        query: impl AsRef<str>,
        variables: Value,
    ) -> GraphQLResponse {
        self.send_query(query, Some(variables)).await
    }
}

pub struct GraphQLResponse {
    pub body: Value,
}

impl GraphQLResponse {
    pub fn assert_no_errors(&self) {
        assert_eq!(self.body["errors"], Value::Null);
    }

    pub fn get(&self, pointer: impl AsRef<str>) -> &Value {
        let pointer = pointer.as_ref();
        self.body.pointer(pointer).unwrap_or_else(|| {
            panic!(
                "No value found at {pointer}, body: {}",
                to_string_pretty(&self.body).unwrap()
            )
        })
    }

    pub fn get_string(&self, pointer: impl AsRef<str>) -> &str {
        let pointer = pointer.as_ref();
        let value = self.get(pointer);

        value.as_str().unwrap_or_else(|| {
            panic!(
                "Value at {pointer} was not a string: {}",
                to_string_pretty(value).unwrap()
            )
        })
    }
}
