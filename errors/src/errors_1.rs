use actix_web::{error, get, Result};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmnt = "my error: {}", name)]
struct MyError {
    name: &'static str,
}

impl error::ResponseError for MyError {}

#[get("/error1")]
pub async fn index() -> Result<&'static str, MyError> {
    Err(MyError { name: "test" })
}
