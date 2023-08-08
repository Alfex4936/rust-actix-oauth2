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
use std::{fs::File, io, io::BufReader};

use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let tls_config = load_rustls_config();

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
    .bind_rustls("127.0.0.1:8080", tls_config)?
    // .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn open_first_available(filenames: &[&str]) -> io::Result<BufReader<File>> {
    for filename in filenames {
        if let Ok(file) = File::open(filename) {
            return Ok(BufReader::new(file));
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "None of the files could be opened",
    ))
}

fn load_rustls_config() -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let mut cert_file = open_first_available(&["cert.pem", "localhost.pem"])
        .expect("Failed to open certificate file");
    let mut key_file =
        open_first_available(&["localhost-key.pem", "key.pem"]).expect("Failed to open key file");

    // convert files to key/cert objects
    let cert_chain = certs(&mut cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(&mut key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
