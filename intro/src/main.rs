mod app_setup;
mod request_derived;
mod request_manual;
mod state;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = web::Data::new(state::SharedAppState {
        phase: std::sync::Mutex::new(false),
    });
    HttpServer::new(move || {
        App::new()
            .data(state::MainAppState {
                name: "Learning Actix Web".to_string(),
            })
            .app_data(shared_data.clone())
            .service(app_setup::create_scope_3())
            .service(app_setup::create_scope_2())
            // root scope ("/" or "") should defined last,
            // otherwaise will try to handle all requests
            .service(app_setup::create_scope_1())
    })
    .bind("127.0.0.1:4001")?
    .run()
    .await
}
