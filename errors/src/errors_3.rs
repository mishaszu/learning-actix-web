use actix_web::{error, get, Result};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "Custom error")]
struct MyError {
    name: &'static str,
}

#[get("/error5")]
pub async fn error_5() -> Result<&'static str> {
    let result: Result<&'static str, MyError> = Err(MyError { name: "test" });
    Ok(result.map_err(|e| error::ErrorBadRequest(e.name))?)
}
