mod extractors;
mod handler;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(extractors::extractor_1)
            .service(extractors::extractor_2)
            .service(extractors::extractor_3)
            .service(web::scope("/").route("", web::get().to(handler::basic)))
    })
    .bind("127.0.0.1:4001")?
    .run()
    .await
}
