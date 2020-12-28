use super::request_derived::{echo, hello, main_state};
use super::request_manual::{counter, counter_post, multiple_params, status, status_post};
use super::state::CounterAppState;
use actix_web::{guard, web, Scope};

pub fn scope_1(cfg: &mut web::ServiceConfig) {
    // Important: scope prefix is glued to all request,
    // so request '/hello' with scope '/' will result with '//hello' endpoint
    // to create '/hello' endpoint scope have to be '' or request have to be 'hello'
    cfg.service(
        web::scope("")
            .service(hello)
            .service(echo)
            .service(main_state),
    );
}

pub fn scope_2(cfg: &mut web::ServiceConfig) {
    cfg.data(CounterAppState {
        counter: std::sync::Mutex::new(0),
    })
    .service(
        web::scope("/counter")
            .route("", web::get().to(counter).guard(guard::Get()))
            .route("", web::post().to(counter_post).guard(guard::Post())),
    )
    .service(
        web::scope("/status")
            .route("", web::get().to(status).guard(guard::Get()))
            .route("", web::post().to(status_post).guard(guard::Post())),
    )
    .service(web::scope("/multi").route("", web::post().to(multiple_params)));
}
