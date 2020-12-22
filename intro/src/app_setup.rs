use super::request_derived::{counter, counter_post, echo, hello, main_state, status, status_post};
use super::request_manual::hello_manual;
use super::state::CounterAppState;
use actix_web::{web, Scope};

pub fn create_scope_1() -> Scope {
    // Important: scope prefix is glued to all request,
    // so request '/hello' with scope '/' will result with '//hello' endpoint
    // to create '/hello' endpoint scope have to be '' or request have to be 'hello'
    web::scope("")
        .service(hello)
        .service(echo)
        .service(main_state)
}

pub fn create_scope_2() -> Scope {
    web::scope("/hello").route("", web::get().to(hello_manual))
}

fn scope_3_configure(cfg: &mut web::ServiceConfig) {
    cfg.data(CounterAppState {
        counter: std::sync::Mutex::new(0),
    });
}

pub fn create_scope_3() -> Scope {
    web::scope("/state")
        .configure(scope_3_configure)
        .service(counter)
        .service(counter_post)
        .service(status)
        .service(status_post)
}
