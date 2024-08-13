use actix_web::{
    dev, error::ResponseError, http::header, middleware::ErrorHandlerResponse, HttpResponse, Result,
};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use uuid::Error as ParseError;

use crate::users::response::GenericResponse;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display("Internal Server Error")]
    InternalServerError,

    #[display("BadRequest: {_0}")]
    BadRequest(String),

    #[display("Unauthorized")]
    Unauthorized,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json(GenericResponse {
                    status: "500 Internal Server Error".to_string(),
                    message: "Internal Server Error, Please try later".to_string(),
                })
            }
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(GenericResponse {
                    status: "400 Bad Request".to_string(),
                    message: message.to_string(),
                })
            }
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json(GenericResponse {
                status: "401 Unauthorized".to_string(),
                message: "Unauthorized".to_string(),
            }),
        }
    }
}

// we can return early in our handlers if UUID provided by the user is not valid
// and provide a custom message
impl From<ParseError> for ServiceError {
    fn from(_: ParseError) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_owned();
                    return ServiceError::BadRequest(message);
                }
                ServiceError::InternalServerError
            }
            _ => ServiceError::InternalServerError,
        }
    }
}
