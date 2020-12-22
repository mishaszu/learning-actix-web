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

// "/state"
#[get("/counter")]
async fn counter(data: web::Data<CounterAppState>) -> String {
    format!("Current counter value: {}", &data.counter.lock().unwrap())
}

#[post("/counter")]
async fn counter_post(data: web::Data<CounterAppState>, req_body: String) -> impl Responder {
    let number = match req_body.parse::<i32>() {
        Ok(value) => value,
        _ => 1,
    };
    let mut value = data.counter.lock().unwrap();
    *value += number;
    HttpResponse::Ok()
}

#[get("/status")]
async fn status(
    data_1: web::Data<MainAppState>,
    data_2: web::Data<CounterAppState>,
    data_3: web::Data<SharedAppState>,
) -> String {
    format!(
        "App: {}, counter: {}, state: {}",
        &data_1.name,
        &data_2.counter.lock().unwrap(),
        data_3.phase.lock().unwrap().to_string()
    )
}

#[post("/status")]
async fn status_post(data: web::Data<SharedAppState>) -> impl Responder {
    let mut state = data.phase.lock().unwrap();
    *state = true;
    HttpResponse::Ok()
}
