use actix_web::http::StatusCode;
use serde::Serialize;
use argon2;

#[derive(Serialize)]
pub struct Error {
    status: u16,
    message: String,
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: e.to_string(),
        }
    }
}

impl From<argon2::Error> for Error {
    fn from(e: argon2::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: e.to_string(),
        }
    }
}

impl From<scylla::transport::errors::QueryError> for Error {
    fn from(e: scylla::transport::errors::QueryError) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: e.to_string(),
        }
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(e: validator::ValidationErrors) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: e.to_string(),
        }
    }
}

//
impl From<String> for Error {
    fn from(e: String) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: e,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: e.to_string(),
        }
    }
}

impl From<actix_web::Error> for Error {
    fn from(e: actix_web::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: e.to_string(),
        }
    }
}