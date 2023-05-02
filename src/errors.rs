use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::Display;
use serde::Serialize;
use std::{convert::Infallible, error::Error as StdError};

/* Error Handler */
#[derive(Debug, Display)]
pub enum LynixError {
    #[display(fmt = "Internal error")]
    InternalError,

    #[display(fmt = "Bad request")]
    BadClientData,

    #[display(fmt = "Not found")]
    NotFound,

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Bad data provided")]
    BadData(String),
}

impl ResponseError for LynixError {
    fn error_response(&self) -> HttpResponse {
        let error_message = ErrorMessage {
            err: self.to_string(),
            status_code: self.status_code().as_u16(),
            message: match self {
                LynixError::BadData(msg) => msg.to_string(),
                _ => self.to_string(),
            },
            current_timestamp: chrono::Local::now().to_string(),
        };
        
        let json_body = serde_json::to_string(&error_message).unwrap();

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(json_body)
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            LynixError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
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

impl From<std::option::Option<Infallible>> for LynixError {
    fn from(_: std::option::Option<Infallible>) -> Self {
        LynixError::InternalError
    }
}

#[derive(Serialize)]
struct ErrorMessage {
    err: String,
    status_code: u16,
    message: String,
    current_timestamp: String,
}