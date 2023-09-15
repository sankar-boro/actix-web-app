use std::num::ParseIntError;

use argon2;
use serde_json;
use serde::Serialize;

use actix_web::{
    http::StatusCode,
    HttpResponse,
};
use derive_more::Display;
use actix_session::{SessionInsertError, SessionGetError};

#[derive(Display, Debug)]
#[display(fmt = "status: {}", status)]
pub struct Error {
    status: StatusCode,
    message: String,
}

impl Error {
    pub fn get_status(&self) -> StatusCode {
        self.status
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<argon2::Error> for Error {
    fn from(e: argon2::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(e: validator::ValidationErrors) -> Self {
        let x = e.errors();
        if x.contains_key("email") {
            return Error {
                status: StatusCode::from_u16(400).unwrap(),
                message: "invalid_email".to_string(),
            };
        }
        if x.contains_key("password") {
            return Error {
                status: StatusCode::from_u16(400).unwrap(),
                message: "invalid_password".to_string(),
            };
        }
        if x.contains_key("fname") {
            return Error {
                status: StatusCode::from_u16(400).unwrap(),
                message: "invalid_fname".to_string(),
            };
        }
        if x.contains_key("lname") {
            return Error {
                status: StatusCode::from_u16(400).unwrap(),
                message: "invalid_lname".to_string(),
            };
        }
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

//
impl From<String> for Error {
    fn from(e: String) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e,
        }
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<actix_web::Error> for Error {
    fn from(e: actix_web::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<uuid::Error> for Error {
    fn from(e: uuid::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}


impl From<SessionInsertError> for Error {
    fn from(e: SessionInsertError) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<SessionGetError> for Error {
    fn from(e: SessionGetError) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(e: tokio_postgres::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<deadpool_postgres::PoolError> for Error {
    fn from(e: deadpool_postgres::PoolError) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    status: u16,
    message: String,
}

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        self.get_status()
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            status: self.status_code().as_u16(),
            message: self.get_message()
        })
    }
}