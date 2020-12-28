use super::state::{CounterAppState, MainAppState, SharedAppState};
use actix_web::{get, post, web, HttpResponse, Responder};

// "/"
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("Backend echo: {}", req_body))
}

#[get("/status")]
async fn main_state(data_1: web::Data<MainAppState>, data_2: web::Data<SharedAppState>) -> String {
    format!(
        "App Name: {} state is: {}",
        &data_1.name,
        data_2.phase.lock().unwrap().to_string()
    )
}
