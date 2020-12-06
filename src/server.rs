use crate::configuration::CONFIG;
use crate::schema;
use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer, Responder};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;
use std::sync::Arc;

pub fn start(listener: TcpListener, db_pool: PgPool) -> io::Result<actix_web::dev::Server> {
    // Initialising env-logger multiple times panics, which breaks tests
    // std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();

    let sch = web::Data::new(Arc::new(schema::create_schema()));
    let ctx = web::Data::new(Arc::new(schema::Context { db: db_pool }));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(sch.clone())
            .app_data(ctx.clone())
            .wrap(middleware::Logger::default())
            .service(graphql)
            .service(graphiql)
            .service(health_check)
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[post("/graphql")]
async fn graphql(
    sch: web::Data<Arc<schema::Schema>>,
    ctx: web::Data<Arc<schema::Context>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ret = data.execute(&sch, &ctx).await;
    let json_str = serde_json::to_string(&ret)?;
    let res = HttpResponse::Ok()
        .content_type("application/json")
        .body(json_str);

    Ok(res)
}

#[get("/graphiql")]
async fn graphiql() -> HttpResponse {
    let html = graphiql_source(
        &format!(
            "http://{}:{}/graphql",
            CONFIG.server_host, CONFIG.server_port
        ),
        None,
    );

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
