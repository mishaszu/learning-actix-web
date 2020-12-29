#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_json;

use actix_http::{body::Body, Response};
use actix_session::{CookieSession, Session};
use actix_web::dev::ServiceResponse;
use actix_web::http::{header, StatusCode};
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Result};

use handlebars::Handlebars;

use std::io;

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>, session: Session) -> Result<HttpResponse> {
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        counter = count;
    } else {
        session.set("counter", counter)?;
    };
    let data = json!({
        "name": "Testing session",
        "counter": counter
    });
    let body = hb.render("index", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}

#[post("/")]
async fn post_index(session: Session) -> Result<HttpResponse> {
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        counter = count + 1;
        session.set("counter", counter)?;
    } else {
        session.set("counter", counter)?;
    }
    Ok(HttpResponse::Ok().json(json!({ "new_value": counter })))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web:info");
    env_logger::init();
    println!("Starting http server: 127.0.0.1:8080");

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .wrap(error_handlers())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(Logger::default())
            .app_data(handlebars_ref.clone())
            .service(index)
            .service(post_index)
    })
    .bind("127.0.0.1:4001")?
    .run()
    .await
}

fn error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> Response<Body> {
    let request = res.request();

    let fallback = |e: &str| {
        Response::build(res.status())
            .content_type("text/plain")
            .body(e.to_string())
    };

    let hb = request
        .app_data::<web::Data<Handlebars>>()
        .map(|x| x.get_ref());

    match hb {
        Some(hb) => {
            let data = json!({
                "error": error,
                "status_code": res.status().as_str()
            });
            let body = hb.render("error", &data);

            match body {
                Ok(body) => Response::build(res.status())
                    .content_type("text/html")
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}
