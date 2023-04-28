use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    App, HttpResponse,
};
use derive_more::{Display, Error};
use std::error::Error as StdError;

/* Error Handler */
#[derive(Debug, Display)]
pub enum LynixError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,

    #[display(fmt = "not found")]
    NotFound,

    #[display(fmt = "unauthorized")]
    Unauthorized,

    #[display(fmt = "bad data")]
    BadData(String),
}

impl ResponseError for LynixError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            LynixError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            LynixError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            LynixError::NotFound => StatusCode::NOT_FOUND,
            LynixError::Unauthorized => StatusCode::UNAUTHORIZED,
            LynixError::BadClientData => StatusCode::BAD_REQUEST,
            LynixError::BadData(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl StdError for LynixError {}

impl From<mongodb::error::Error> for LynixError {
    fn from(_: mongodb::error::Error) -> Self {
        LynixError::InternalError
    }
}

impl From<base64::DecodeError> for LynixError {
    fn from(_: base64::DecodeError) -> Self {
        LynixError::InternalError
    }
}

/* Catch all */
impl From<actix_web::Error> for LynixError {
    fn from(_: actix_web::Error) -> Self {
        LynixError::InternalError
    }
}