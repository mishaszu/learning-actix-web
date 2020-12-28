use actix_web::{error, get, Result};
use derive_more::{Display, Error};
use log::debug;

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
struct MyError {
    name: &'static str,
}

impl error::ResponseError for MyError {}

#[get("/error1")]
pub async fn error_1() -> Result<&'static str, MyError> {
    let err = Err(MyError { name: "test" });
    debug!("{:?}", err);
    err
}
