mod config;

mod auth;
mod handlers;
mod models;

mod responses;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let db = AppState::init();
    let app_data = web::Data::new(db);
    let public_dir = std::env::current_dir().unwrap().join("public");

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&app_data.env.client_origin)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        // let cors = Cors::permissive();
        App::new()
            .app_data(app_data.clone())
            .service(actix_files::Files::new("/api/images", &public_dir))
            .configure(handlers::auth_handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
