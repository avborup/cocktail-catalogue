use std::io;
use std::sync::Arc;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, Responder};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::schema;

const HOST: &str = "127.0.0.1:8080";

async fn graphiql() -> HttpResponse {
    let html = graphiql_source(&format!("http://{}/graphql", HOST));

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    sch: web::Data<Arc<schema::Schema>>,
    ctx: web::Data<Arc<schema::Context>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        let ret = data.execute(&sch, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&ret)?)
    })
    .await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn start(schema: Arc<schema::Schema>, ctx: Arc<schema::Context>) -> io::Result<actix_web::dev::Server> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Server started at {}", HOST);

    let server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(ctx.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .route("/health_check", web::get().to(health_check))
    })
    .bind(HOST)?
    .run();

    Ok(server)
}

