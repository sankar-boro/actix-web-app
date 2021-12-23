use argon2::{Config};
use crate::AppError;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionClaims {
	id: String,
	email: String,
	fname: String,
	lname: String,
	exp: i64,
	iat: i64,
}

impl SessionClaims {
	pub fn new(id: Uuid, email: String, fname: String, lname: String, exp: i64, iat: i64) -> Self {
		SessionClaims {
			id: id.to_string(),
			email,
			fname,
			lname,
			exp,
			iat
		}
	}

	#[allow(dead_code)]
	pub fn get_id(&self) -> &str {
		&self.id
	}
}

pub fn validate_user_credentials(req_pass: &str, db_pass: &[u8]) -> Result<(), actix_web::Error> {
    let salt = b"sankar_boro";
    let config = Config::default();
    let req_pass = req_pass.as_bytes();
    match argon2::hash_encoded(req_pass, salt, &config) {
      	Ok(data) => {
          	if data.as_bytes() != db_pass {
              	return Err(AppError::from("Invalid credentials").into());
          	}
          	Ok(())
      	},
      	Err(err) => Err(AppError::from(err).into())
    }
}