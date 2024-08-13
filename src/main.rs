use actix_identity::IdentityMiddleware;
use actix_web::{
    http::StatusCode,
    middleware::{self, ErrorHandlers},
    web, App, HttpServer,
};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager, Pool},
};
use errors::add_error_header;

mod errors;
mod schema;
mod users;
mod utils;

fn get_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

fn get_port() -> u16 {
    std::env::var("PORT")
        .unwrap_or("8080".to_owned())
        .parse()
        .expect("PORT must be number")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = get_pool();
    let domain = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_owned());

    let port: u16 = get_port();
    log::info!("starting HTTP server at http://{domain}:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(users::handlers::config)
            .wrap(IdentityMiddleware::default())
            .wrap(middleware::Logger::default())
            .wrap(ErrorHandlers::new().handler(StatusCode::BAD_REQUEST, add_error_header))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
