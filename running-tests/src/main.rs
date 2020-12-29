use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().body("test")
}

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct {
    name: String,
}

async fn post_index(req: web::Json<MyStruct>) -> HttpResponse {
    HttpResponse::Ok().json(MyStruct {
        name: format!("updated: {}", req.name),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/")
                .route("", web::get().to(get_index))
                .route("", web::post().to(post_index)),
        )
    })
    .bind("127.0.0.1:4001")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{dev::Body, http, test};
    use bytes::Bytes;

    #[actix_rt::test]
    async fn test_index_ok() {
        let mut app = test::init_service(App::new().route("/", web::get().to(get_index))).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let result = test::read_response(&mut app, req).await;
        assert_eq!(result, web::Bytes::from_static(b"test"));
    }

    #[actix_rt::test]
    async fn test_index_without_app_creation() {
        let mut res = get_index().await;
        let bytes = web::Bytes::from("test");
        let value = Body::Bytes(bytes);
        assert_eq!(res.take_body().as_ref().unwrap(), &value);
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;
    use actix_web::{test, web};
    use serde::{Deserialize, Serialize};

    #[actix_rt::test]
    async fn test_index_body() {
        let mut app = test::init_service(
            App::new().service(web::scope("/").route("", web::get().to(get_index))),
        )
        .await;
        let req = test::TestRequest::with_uri("/").to_request();
        let res = test::call_service(&mut app, req).await;
        let body = test::read_body(res).await;
        let value = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(value, "test".to_string());
    }

    #[actix_rt::test]
    async fn test_index_post_body() {
        let send_struct = MyStruct {
            name: "test".to_string(),
        };
        let test_struct = MyStruct {
            name: "updated: test".to_string(),
        };
        let mut app = test::init_service(
            App::new().service(web::scope("/").route("", web::post().to(post_index))),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&send_struct)
            .to_request();
        let res = test::call_service(&mut app, req).await;
        let json: MyStruct = test::read_body_json(res).await;
        assert_eq!(json.name, test_struct.name);
    }
}
