use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    PATCH,
    OTHER(String),
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum HttpStatusCode {
    #[default]
    Success = 200,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    Conflict = 409,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
}

#[derive(Debug, Clone)]
pub enum HttpError {
    BadRequest(HttpStatusCode, &'static str),
    Unauthorized(HttpStatusCode, &'static str),
    Forbidden(HttpStatusCode, &'static str),
    NotFound(HttpStatusCode, &'static str),
    MethodNotAllowed(HttpStatusCode, &'static str),
    NotAcceptable(HttpStatusCode, &'static str),
    Conflict(HttpStatusCode, &'static str),
    InternalServerError(HttpStatusCode, &'static str),
    NotImplemented(HttpStatusCode, &'static str),
    BadGateway(HttpStatusCode, &'static str),
    ServiceUnavailable(HttpStatusCode, &'static str),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpError::BadRequest(err_code, msg) => {
                write!(f, "[ERROR] {}: Bad Request {}", *err_code as u16, msg)
            }
            HttpError::Unauthorized(err_code, msg) => {
                write!(f, "[ERROR] {}: Unauthorized {}", *err_code as u16, msg)
            }
            HttpError::Forbidden(err_code, msg) => {
                write!(f, "[ERROR] {}: Forbidden {}", *err_code as u16, msg)
            }
            HttpError::NotFound(err_code, msg) => {
                write!(f, "[ERROR] {}: Not Found {}", *err_code as u16, msg)
            }
            HttpError::MethodNotAllowed(err_code, _) => {
                write!(f, "[ERROR] {}: Method Not Allowed", *err_code as u16)
            }
            HttpError::NotAcceptable(err_code, msg) => {
                write!(f, "[ERROR] {}: Not Acceptable {}", *err_code as u16, msg)
            }
            HttpError::Conflict(err_code, _) => write!(f, "[ERROR] {}: Conflict", *err_code as u16),
            HttpError::InternalServerError(err_code, msg) => {
                write!(
                    f,
                    "[ERROR] {}: Internal Server Error {}",
                    *err_code as u16, msg
                )
            }
            HttpError::NotImplemented(err_code, msg) => {
                write!(f, "[ERROR] {}: Not Implemented {}", *err_code as u16, msg)
            }
            HttpError::BadGateway(err_code, _) => {
                write!(f, "[ERROR] {}: Bad Gateway", *err_code as u16)
            }
            HttpError::ServiceUnavailable(err_code, msg) => {
                write!(
                    f,
                    "[ERROR] {}: Service Unavailable {}",
                    *err_code as u16, msg
                )
            }
        }
    }
}

impl std::error::Error for HttpError {}
