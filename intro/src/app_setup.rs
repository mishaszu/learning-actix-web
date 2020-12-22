use super::request_derived::{echo, hello};
use super::request_manual::hello_manual;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Scope};

pub fn create_scope_1() -> Scope {
    // Important: scope prefix is glued to all request,
    // so request '/hello' with scope '/' will result with '//hello' endpoint
    // to create '/hello' endpoint scope have to be '' or request have to be 'hello'
    web::scope("").service(hello).service(echo)
}

pub fn create_scope_2() -> Scope {
    web::scope("/hello").route("", web::get().to(hello_manual))
}
