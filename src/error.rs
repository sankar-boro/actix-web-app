use argon2;
use serde_json;

use actix_web::{
    http::{
        header::ContentType, 
        StatusCode
    },
    HttpResponse,
};
use derive_more::Display;


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

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
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


impl From<scylla::transport::errors::QueryError> for Error {
    fn from(e: scylla::transport::errors::QueryError) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(e: validator::ValidationErrors) -> Self {
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

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        self.get_status()
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::html())
        .body(self.get_message())
    }
}