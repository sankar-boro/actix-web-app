use argon2::{self, Config};
use crate::AppError;

pub fn encrypt_text(user_password: &str) -> Result<String, actix_web::Error> {
    let salt = b"sankar_boro";
    let config = Config::default();
    match argon2::hash_encoded(user_password.as_bytes(), salt, &config) {
      Ok(data) => Ok(data),
      Err(err) => Err(AppError::from(err).into())
    }
}
