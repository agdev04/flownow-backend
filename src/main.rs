mod schema;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use config::get_auth_setup;
use guard::guard_config;
use users::handler::create_user;

pub mod auth;
pub mod categories;
pub mod config;
pub mod db;
pub mod guard;
pub mod meditations;
pub mod upload;
pub mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    HttpServer::new(move || {
        let cors = Cors::default()
            // .allowed_origin("http://localhost:5173")
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .supports_credentials();

        App::new()
            .wrap(cors)
            .configure(match get_auth_setup().as_str() {
                "http_only" => auth::http_only::route::auth_http_only_config,
                _ => auth::default::route::auth_default_config,
            })
            .route("/register", web::post().to(create_user))
            .configure(guard_config)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 3005))?
    .run()
    .await
}
