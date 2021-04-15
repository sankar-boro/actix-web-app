use jsonwebtoken;
use std::fmt::Display;
use serde::{Serialize, Deserialize};
use actix_web::error::Error as ActixWebError;
use {serde_json, serde_json::{Value as JsonValue}};

#[derive(Debug, Serialize)]
pub enum LoonyError {
  InternalServerError(JsonValue),
  ParseError(JsonValue),
  ServiceError(JsonValue),
  ActixWebError(JsonValue),
  JSONWebTokenError(JsonValue),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoonyResponse {
  pub status_code: Option<String>,
  pub message: String, 
}

impl From<LoonyResponse> for serde_json::Value {
  fn from(res: LoonyResponse) -> Self {
    res.into()
  }
}

impl Display for LoonyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      LoonyError::InternalServerError(res) => {
        return write!(f, "{}", res)
      }
      LoonyError::ParseError(res) => {
        return write!(f, "{}", res)
      }
      LoonyError::ServiceError(res) => {
        return write!(f, "{}", res)
      }
      LoonyError::ActixWebError(res) => {
        return write!(f, "{}", res)
      }
      LoonyError::JSONWebTokenError(res) => {
        return write!(f, "{}", res)
      }
    }
  }
}

impl From<r2d2::Error> for LoonyError {
  fn from(err: r2d2::Error) -> Self {
    let res = LoonyResponse {
      status_code: None,
      message: err.to_string(), 
    };
    let str_err = serde_json::to_string(&res).unwrap();
    let v: JsonValue = serde_json::from_str(&str_err).unwrap();
    LoonyError::InternalServerError(v)
  }
}

impl From<diesel::result::Error> for LoonyError {
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
    let res = LoonyResponse {
      status_code: None,
      message: match_err.to_string(), 
    };
    let res_str = serde_json::to_string(&res).unwrap();
    let json_value: JsonValue = serde_json::from_str(&res_str).unwrap();
    LoonyError::InternalServerError(json_value)
  }
}

impl From<std::num::ParseIntError> for LoonyError {
  fn from(err:std::num::ParseIntError) -> Self {
    let res = LoonyResponse {
      status_code: None,
      message: err.to_string(), 
    };
    let res_str = serde_json::to_string(&res).unwrap();
    let json_value: JsonValue = serde_json::from_str(&res_str).unwrap();
    LoonyError::InternalServerError(json_value)
  }
}

impl From<redis::Connection> for LoonyError {
  fn from(_:redis::Connection) -> Self {
    let res = LoonyResponse {
      status_code: None,
      message: "redis connection error".to_string(), 
    };
    let res_str = serde_json::to_string(&res).unwrap();
    let json_value: JsonValue = serde_json::from_str(&res_str).unwrap();
    LoonyError::InternalServerError(json_value)
  }
}

impl From<String> for LoonyError {
  fn from(err:String) -> Self {
    let res = LoonyResponse {
      status_code: None,
      message: err.to_string(), 
    };
    let res_str = serde_json::to_string(&res).unwrap();
    let json_value: JsonValue = serde_json::from_str(&res_str).unwrap();
    LoonyError::InternalServerError(json_value)
  }
}

impl From<ActixWebError> for LoonyError {
  fn from(err: ActixWebError) -> Self {
    let res = LoonyResponse {
      status_code: None,
      message: err.to_string(), 
    };
    let res_str = serde_json::to_string(&res).unwrap();
    let json_value: JsonValue = serde_json::from_str(&res_str).unwrap();
    LoonyError::ActixWebError(json_value)
  }
}

impl From<jsonwebtoken::errors::Error> for LoonyError {
  fn from(err: jsonwebtoken::errors::Error) -> Self {
    let res = LoonyResponse {
      status_code: None,
      message: err.to_string(), 
    };
    let res_str = serde_json::to_string(&res).unwrap();
    let json_value: JsonValue = serde_json::from_str(&res_str).unwrap();
    LoonyError::JSONWebTokenError(json_value) 
  }
}