use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DbError};
use std::convert::From;
use uuid::Error as UuidError;

#[derive(Debug, Display, Clone)]
pub enum AuthError {
    #[display(fmt = "DuplicateValue: {}", _0)]
    DuplicateValue(String),

    #[display(fmt = "BadId")]
    BadId,

    #[display(fmt = "GenericError: {}", _0)]
    GenericError(String),

    #[display(fmt = "AuthenticationError: {}", _0)]
    AuthenticationError(String),

    #[display(fmt = "ProcessFailed: {}", _0)]
    ProcessError(String),

    #[display(fmt = "Not found: {}", _0)]
    NotFound(String),
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::BadId => HttpResponse::BadRequest().json("Invalid ID"),
            AuthError::DuplicateValue(ref message) => HttpResponse::BadRequest().json(message),
            AuthError::GenericError(ref message) => HttpResponse::BadRequest().json(message),
            AuthError::AuthenticationError(ref message) => {
                HttpResponse::Unauthorized().json(message)
            }
            AuthError::ProcessError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            AuthError::NotFound(ref message) => HttpResponse::NotFound().json(message),
        }
    }
}

impl From<UuidError> for AuthError {
    fn from(_: UuidError) -> Self {
        AuthError::BadId
    }
}

impl From<DbError> for AuthError {
    fn from(error: DbError) -> Self {
        match error {
            DbError::DatabaseError(kind, info) => {
                let message = info.details().unwrap_or_else(|| info.message()).to_string();
                match kind {
                    DatabaseErrorKind::UniqueViolation => AuthError::DuplicateValue(message),
                    _ => AuthError::GenericError(message),
                }
            }
            _ => AuthError::GenericError(String::from("Some database error occured")),
        }
    }
}
