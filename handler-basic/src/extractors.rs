use actix_web::{get, post, web, HttpRequest, Result};
use serde::Deserialize;

#[get("/data/{data_id}/{value}")]
async fn extractor_1(web::Path((data_id, value)): web::Path<(u32, String)>) -> Result<String> {
    Ok(format!(
        "1) Extracted data id: {}, and data value: {}",
        data_id, value,
    ))
}

#[derive(Deserialize)]
struct MyInfo {
    user_id: u32,
    value: String,
}

#[get("/data/2/{user_id}/{value}")]
async fn extractor_2(info: web::Path<MyInfo>) -> Result<String> {
    Ok(format!(
        "2) Extracted user id: {}, and data value: {}",
        info.user_id, info.value,
    ))
}

#[derive(Deserialize)]
struct PostData {
    value: String,
    number: i32,
    phrase: String,
}

#[post("/data/3/{user_id}")]
async fn extractor_3(
    web::Path(user_id): web::Path<u32>,
    req: web::Json<PostData>,
) -> Result<String> {
    Ok(format!(
        "3) For user: {} value is: {}, number is: {}, and phrase is: {}",
        user_id, req.value, req.number, req.phrase
    ))
}
