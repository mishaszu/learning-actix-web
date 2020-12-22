use actix_web::{HttpResponse, Responder};

pub async fn hello_manual() -> impl Responder {
    HttpResponse::Ok().body("Manual Hello")
}
