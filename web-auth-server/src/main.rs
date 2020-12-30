#[macro_use]
extern crate diesel;
extern crate lettre;
extern crate native_tls;
extern crate serde_json;

mod errors;
mod models;
mod schema;
mod vars;

use actix_cors::Cors;
use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{http::header, middleware, web, App, HttpServer};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=infoctix_server=info");
    let manager = ConnectionManager::<PgConnection>::new(vars::database_url());
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create a database connection pool");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(
                CookieSession::signed(&[0, 32])
                    .domain(vars::domain_url().as_str())
                    .name("auth")
                    .secure(false),
            )
            .wrap(
                Cors::default()
                    .allowed_origin("*")
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .max_age(3600),
            )
            .service(Files::new("/assets", "./templates/assets"))
            .wrap(middleware::Logger::default())
    })
    .bind(format!("{}:{}", vars::domain(), vars::port()))?
    .run()
    .await
}
