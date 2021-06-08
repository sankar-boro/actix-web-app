use argon2::{Config};
use crate::AppError;

pub fn validate_password(req_pass: &str, db_pass: &[u8]) -> Result<(), actix_web::Error> {
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