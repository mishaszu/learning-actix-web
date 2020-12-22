mod app_setup;
mod request_derived;
mod request_manual;
use actix_web::{web, App, HttpServer};
use request_derived::{echo, hello};
use request_manual::hello_manual;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(app_setup::create_scope_2())
            // root scope ("/" or "") should defined last, otherwaise will try to handle all
            // requests
            .service(app_setup::create_scope_1())
    })
    .bind("127.0.0.1:4001")?
    .run()
    .await
}
