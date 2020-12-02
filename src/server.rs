use crate::configuration::CONFIG;
use crate::schema;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, Responder};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use sqlx::PgPool;
use std::io;
use std::net::TcpListener;
use std::sync::Arc;

async fn graphiql() -> HttpResponse {
    let html = graphiql_source(&format!(
        "http://{}:{}/graphql",
        CONFIG.server_host, CONFIG.server_port
    ));

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

pub fn start(listener: TcpListener, db_pool: PgPool) -> io::Result<actix_web::dev::Server> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let sch = web::Data::new(Arc::new(schema::create_schema()));
    let ctx = web::Data::new(Arc::new(schema::Context { db: db_pool }));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(sch.clone())
            .app_data(ctx.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
