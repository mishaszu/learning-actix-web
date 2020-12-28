use actix_web::{middleware::Logger, App, HttpServer};
mod errors_1;
mod errors_2;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(errors_1::error_1)
            .service(errors_2::error_2)
            .service(errors_2::error_3)
            .service(errors_2::error_4)
    })
    .bind("127.0.0.1:4001")?
    .run()
    .await
}
