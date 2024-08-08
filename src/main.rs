use actix_identity::IdentityMiddleware;
use actix_web::{middleware, web, App, HttpServer};
use diesel::{r2d2, PgConnection};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let domain = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_owned());

    let port: u16 = std::env::var("PORT").unwrap_or(String::from("8080")).parse().expect("PORT must be number");
    log::info!("starting HTTP server at http://{domain}:{port}");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
            )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
