#[macro_use]
extern crate diesel;
extern crate lettre;
extern crate native_tls;
extern crate serde_json;

mod auth_handler;
mod email_service;
mod errors;
mod models;
mod password_handler;
mod register_handler;
mod schema;
mod templates;
mod utils;
mod vars;

use actix_cors::Cors;
use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{guard, http::header, middleware, web, App, HttpServer};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    let manager = ConnectionManager::<PgConnection>::new(vars::database_url());
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create a database connection pool");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(
                CookieSession::signed(&[0; 32])
                    .domain(vars::domain_url().as_str())
                    .name("auth")
                    .secure(false),
            )
            .wrap(
                Cors::default()
                    .allowed_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .service(Files::new("/assets", "./templates/assets"))
            .service(
                web::scope("/")
                    .service(
                        web::resource("/register")
                            .route(web::get().to(register_handler::show_confirmation_form))
                            .route(
                                web::post()
                                    .guard(guard::Header("content-type", "application/json"))
                                    .to(register_handler::send_confirmation),
                            )
                            .route(
                                web::post()
                                    .guard(guard::Header(
                                        "content-type",
                                        "application/x-www-form-urlencoded",
                                    ))
                                    .to(register_handler::send_confirmation_for_browser),
                            ),
                    )
                    .service(
                        web::resource("/register/{path_id}")
                            .route(web::get().to(password_handler::show_password_form))
                            .route(
                                web::post()
                                    .guard(guard::Header(
                                        "content-type",
                                        "application/x-www-form-urlencoded",
                                    ))
                                    .to(password_handler::create_account_for_browser),
                            )
                            .route(
                                web::post()
                                    .guard(guard::Header("content-type", "application/json"))
                                    .to(password_handler::create_account),
                            ),
                    )
                    .service(
                        web::resource("/me")
                            .route(web::get().to(auth_handler::me))
                            .route(web::delete().to(auth_handler::sign_out)),
                    )
                    .service(
                        web::resource("/signin")
                            .route(web::get().to(auth_handler::show_sign_in_form))
                            .route(
                                web::post()
                                    .guard(guard::Header("content-type", "application/json"))
                                    .to(auth_handler::sign_in),
                            )
                            .route(
                                web::post()
                                    .guard(guard::Header(
                                        "content-type",
                                        "application/x-www-form-urlencoded",
                                    ))
                                    .to(auth_handler::sign_in_for_browser),
                            ),
                    ),
            )
    })
    .bind(format!("{}:{}", vars::domain(), vars::port()))?
    .run()
    .await
}
