use actix_web::{App, HttpServer};
mod errors_1;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(errors_1::index))
        .bind("127.0.0.1:4001")?
        .run()
        .await
}
