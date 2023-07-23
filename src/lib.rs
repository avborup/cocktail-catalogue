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
use graphql::{MutationRoot, QueryRoot};
use sqlx::SqlitePool;

use crate::graphql::ApiSchema;

pub mod configuration;
mod graphql;
pub mod logging;

pub async fn create_app(db: SqlitePool) -> eyre::Result<Router> {
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

pub async fn run(listener: TcpListener, db: SqlitePool) -> eyre::Result<()> {
    let app = create_app(db).await?;

    tracing::info!("Listening on {}", listener.local_addr()?);

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
