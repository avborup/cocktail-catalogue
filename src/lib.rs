use std::net::TcpListener;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Request, Response, Schema,
};
use axum::{
    debug_handler,
    extract::{FromRef, State},
    http,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use configuration::AppSettings;
use eyre::Context;
use graphql::{MutationRoot, QueryRoot};
use sqlx::SqlitePool;

use crate::graphql::ApiSchema;

pub mod configuration;
mod graphql;
pub mod logging;

pub async fn create_app(config: &AppSettings) -> eyre::Result<Router> {
    let db = SqlitePool::connect(&config.database.connection_string)
        .await
        .wrap_err("Failed to connect to database")?;

    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data::<SqlitePool>(db)
    .finish();

    let server_state = ServerState { schema };

    let router = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/health_check", get(health_check))
        .with_state(server_state)
        .layer(logging::make_http_span_layer());

    Ok(router)
}

#[derive(Clone, FromRef)]
pub struct ServerState {
    schema: ApiSchema,
}

pub async fn run(config: &AppSettings) -> eyre::Result<()> {
    let app = create_app(config).await?;

    let address = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&address)?;

    tracing::info!("Listening on http://{}", address);

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[tracing::instrument(name = "GraphQL", skip_all)]
#[debug_handler(state = ServerState)]
async fn graphql_handler(
    State(schema): State<ApiSchema>,
    Json(request): Json<Request>,
) -> Json<Response> {
    schema.execute(request).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

async fn health_check() -> impl IntoResponse {
    http::StatusCode::OK
}
