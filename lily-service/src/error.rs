use jsonwebtoken;
use std::fmt::Display;
use serde::{Serialize, Deserialize};
use actix_web::error::Error as ActixWebError;
use {serde_json, serde_json::{Value as JsonValue}};

#[derive(Debug, Serialize)]
pub enum lilyError {
  InternalServerError(JsonValue),
  ParseError(JsonValue),
  ServiceError(JsonValue),
  ActixWebError(JsonValue),
  JSONWebTokenError(JsonValue),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct lilyResponse {
  pub status_code: Option<String>,
  pub message: String, 
}

impl From<lilyResponse> for serde_json::Value {
  fn from(res: lilyResponse) -> Self {
    res.into()
  }
}

impl Display for lilyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      lilyError::InternalServerError(res) => {
        return write!(f, "{}", res)
      }
      lilyError::ParseError(res) => {
        return write!(f, "{}", res)
      }
      lilyError::ServiceError(res) => {
        return write!(f, "{}", res)
      }
      lilyError::ActixWebError(res) => {
        return write!(f, "{}", res)
      }
      lilyError::JSONWebTokenError(res) => {
        return write!(f, "{}", res)
      }
    }
  }
}

impl From<r2d2::Error> for lilyError {
  fn from(err: r2d2::Error) -> Self {
    let res = lilyResponse {
      status_code: None,
      message: err.to_string(), 
    };
    let str_err = serde_json::to_string(&res).unwrap();
    let v: JsonValue = serde_json::from_str(&str_err).unwrap();
    lilyError::InternalServerError(v)
  }
}

impl From<diesel::result::Error> for lilyError {
  fn from(err: diesel::result::Error) -> Self {
    let match_err = match err {
        diesel::result::Error::InvalidCString(null_err) => null_err.to_string(),
        diesel::result::Error::DatabaseError(db_err, _) => match db_err {
          diesel::result::DatabaseErrorKind::UniqueViolation => "Unique Violation".to_string(),
          diesel::result::DatabaseErrorKind::ForeignKeyViolation => "Foreign Key Violation".to_string(),
          diesel::result::DatabaseErrorKind::UnableToSendCommand => "Unable To Send Command".to_string(),
          diesel::result::DatabaseErrorKind::SerializationFailure => "Serialization Failure".to_string(),
          diesel::result::DatabaseErrorKind::__Unknown => "Unknown Error".to_string(),
        } 
        diesel::result::Error::NotFound => "Not Found Error".to_string(),
        diesel::result::Error::QueryBuilderError(_) => "Query Builder Error".to_string(),
        diesel::result::Error::DeserializationError(_) => "Deserialization Error".to_string(),
        diesel::result::Error::SerializationError(_) => "Serialization Error".to_string(),
        diesel::result::Error::RollbackTransaction => "Rollback Error".to_string(),
        diesel::result::Error::AlreadyInTransaction => "Aldready Error".to_string(),
        diesel::result::Error::__Nonexhaustive => "Non Exhaustive Error".to_string(),
    };
    let res = lilyResponse {
      status_code: None,
      message: match_err.to_string(), 
    };
    let res_str = serde_json::to_string(&res).unwrap();
    let json_value: JsonValue = serde_json::from_str(&res_str).unwrap();
    lilyError::InternalServerError(json_value)
  }
}

impl From<std::num::ParseIntError> for lilyError {
  fn from(err:std::num::ParseIntError) -> Self {
    let res = lilyResponse {
      status_code: None,
      message: err.to_string(), 
    };
    let res_str = serde_json::to_string(&res).unwrap();
    let json_value: JsonValue = serde_json::from_str(&res_str).unwrap();
    lilyError::InternalServerError(json_value)
  }
}

impl From<redis::Connection> for lilyError {
  fn from(_:redis::Connection) -> Self {
    let res = lilyResponse {
      status_code: None,
      message: "redis connection error".to_string(), 
    };
    let res_str = serde_json::to_string(&res).unwrap();
    let json_value: JsonValue = serde_json::from_str(&res_str).unwrap();
    lilyError::InternalServerError(json_value)
  }
}

impl From<String> for lilyError {
  fn from(err:String) -> Self {
    let res = lilyResponse {
      status_code: None,
      message: err.to_string(), 
    };
    let res_str = serde_json::to_string(&res).unwrap();
    let json_value: JsonValue = serde_json::from_str(&res_str).unwrap();
    lilyError::InternalServerError(json_value)
  }
}

impl From<ActixWebError> for lilyError {
  fn from(err: ActixWebError) -> Self {
    let res = lilyResponse {
      status_code: None,
      message: err.to_string(), 
    };
    let res_str = serde_json::to_string(&res).unwrap();
    let json_value: JsonValue = serde_json::from_str(&res_str).unwrap();
    lilyError::ActixWebError(json_value)
  }
}

impl From<jsonwebtoken::errors::Error> for lilyError {
  fn from(err: jsonwebtoken::errors::Error) -> Self {
    let res = lilyResponse {
      status_code: None,
      message: err.to_string(), 
    };
    let res_str = serde_json::to_string(&res).unwrap();
    let json_value: JsonValue = serde_json::from_str(&res_str).unwrap();
    lilyError::JSONWebTokenError(json_value) 
  }
}