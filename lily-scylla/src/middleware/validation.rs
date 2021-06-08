use std::fmt::Display;
use actix_session::Session;
use jsonwebtoken::Validation;
use jsonwebtoken::DecodingKey;
use crate::utils::SessionClaims;
use actix_web::http::HeaderValue;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, Algorithm};
use {serde_json, serde_json::{Value as JsonValue}};


#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
  status: Option<String>,
  data: String, 
}

#[derive(Debug, Serialize)]
pub enum ValidationError {
  JSONWebTokenError(JsonValue),
  ActixWebError(JsonValue),
  InvalidSessionError(JsonValue),
}

impl From<jsonwebtoken::errors::Error> for ValidationError {
  fn from(err: jsonwebtoken::errors::Error) -> Self {
    let res = Response {
      status: None,
      data: err.to_string(), 
    };
    let str_err = serde_json::to_string(&res).unwrap();
    let v: JsonValue = serde_json::from_str(&str_err).unwrap();
    ValidationError::JSONWebTokenError(v)  
  }
}

impl From<actix_web::Error> for ValidationError {
  fn from(err: actix_web::Error) -> Self {
    let res = Response {
      status: None,
      data: err.to_string(), 
    };
    let str_err = serde_json::to_string(&res).unwrap();
    let v: JsonValue = serde_json::from_str(&str_err).unwrap();
    ValidationError::ActixWebError(v)  
  }
}

impl Display for ValidationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ValidationError::JSONWebTokenError(res) => {
        return write!(f, "{}", res)
      }
      ValidationError::ActixWebError(res) => {
        return write!(f, "{}", res)
      }
      ValidationError::InvalidSessionError(res) => {
        return write!(f, "{}", res)
      }
    }
  }
}

pub struct ValidationHandler<'a> {
  bearer: Option<&'a HeaderValue>
}

impl<'a> ValidationHandler<'a> {
  pub fn new(
    bearer: Option<&'a HeaderValue>
  ) -> Self {
    Self {
      bearer
    }
  }

  pub fn verify_token(&self, session: &Session) -> Result<(), ValidationError> {
    let res= Response {
      status: None,
      data: "Invalid session key".to_string(), 
    };
    let str_err = serde_json::to_string(&res).unwrap();
    let v: JsonValue = serde_json::from_str(&str_err).unwrap();
    if let Some(bearer) = self.bearer {
      return match bearer.to_str() {
        Ok(bearer) => {
          let token = &bearer[7..];
            let claims = decode::<SessionClaims>(
              token,
          &DecodingKey::from_secret("secret".as_bytes()),
              &Validation::new(Algorithm::HS512),
            )?;

            match session.get::<String>(&claims.claims.get_id()) {
              Ok(d) => {
                match d {
                  Some(d) => {
                    if token == d {
                      return Ok(());
                    }
                    Err(ValidationError::InvalidSessionError(v))
                  },
                  None => Err(ValidationError::InvalidSessionError(v))
                }
              },
              Err(_) => Err(ValidationError::InvalidSessionError(v))
            }
        },
        Err(_) => Err(ValidationError::InvalidSessionError(v)),
      };
    }

    Err(ValidationError::InvalidSessionError(v))
  }
}