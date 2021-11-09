use std::fmt::{Debug, Display, Formatter};

use actix_web::{HttpResponse, ResponseError};
use actix_web::error::BlockingError;
use paperclip::actix::api_v2_errors;
use r2d2;
use serde::Serialize;

#[api_v2_errors(code = 400, code = 500)]
pub enum ExpectedError {
    DieselError(String),
    NoneError(String),
}

impl ExpectedError {
    pub fn code(&self) -> String {
        match self {
            ExpectedError::DieselError(_) => String::from("DIESEL_ERROR"),
            ExpectedError::NoneError(_) => String::from("NONE_ERROR"),
        }
    }

    pub fn name(&self) -> String {
        match self {
            ExpectedError::DieselError(_) => String::from("DieselError"),
            ExpectedError::NoneError(_) => String::from("NoneError"),
        }
    }
}

impl From<r2d2::Error> for ExpectedError {
    fn from(err: r2d2::Error) -> Self {
        ExpectedError::DieselError(err.to_string())
    }
}

impl From<diesel::result::Error> for ExpectedError {
    fn from(err: diesel::result::Error) -> Self {
        ExpectedError::DieselError(err.to_string())
    }
}

impl From<BlockingError<diesel::result::Error>> for ExpectedError {
    fn from(err: BlockingError<diesel::result::Error>) -> Self {
        ExpectedError::DieselError(err.to_string())
    }
}

impl Display for ExpectedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedError::DieselError(err) => write!(f, "{}", err),
            ExpectedError::NoneError(err) => write!(f, "{}", err),
        }
    }
}

impl Debug for ExpectedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedError::DieselError(err) => write!(f, "{:?}", err),
            ExpectedError::NoneError(err) => write!(f, "{:?}", err),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: String,
    error: String,
    message: String,
}


impl ResponseError for ExpectedError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: self.code(),
            message: self.to_string(),
            error: self.name(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}