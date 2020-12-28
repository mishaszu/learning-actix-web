use actix_web::{
    dev::HttpResponseBuilder, error, get, http::header, http::StatusCode, HttpResponse, Result,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "internal error")]
    InternalError,
    #[display(fmt = "bad request")]
    BadClientData,
    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

#[get("/error2")]
pub async fn error_2() -> Result<&'static str, MyError> {
    Err(MyError::BadClientData)
}
#[get("/error3")]
pub async fn error_3() -> Result<&'static str, MyError> {
    Err(MyError::InternalError)
}
#[get("/error4")]
pub async fn error_4() -> Result<&'static str, MyError> {
    Err(MyError::Timeout)
}
