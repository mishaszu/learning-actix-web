use actix_web::{
    get, guard, http::header, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};

#[get("/test/")]
async fn index(req: HttpRequest) -> Result<HttpResponse> {
    let url = req.url_for("foo", &["1", "2", "3"])?;
    Ok(HttpResponse::Found()
        .header(header::LOCATION, url.as_str())
        .finish())
}

#[get("/")]
async fn yt(req: HttpRequest) -> impl Responder {
    let url = req.url_for("youtube", &["oHg5SJYRHA0"]).unwrap();
    assert_eq!(url.as_str(), "https://youtube.com/watch/oHg5SJYRHA0");

    url.into_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("test/{a}/{b}/{c}")
                    .name("foo")
                    .guard(guard::Get())
                    .to(|| HttpResponse::Ok()),
            )
            .service(index)
            .service(yt)
            .external_resource("youtube", "https://youtube.com/watch/{video_id}")
    })
    .bind("127.0.0.1:4001")?
    .run()
    .await
}
