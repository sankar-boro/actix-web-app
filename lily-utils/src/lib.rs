mod unique;
pub use unique::time_uuid;

use argon2::{self, Config};
use anyhow::Result;

pub fn encrypt_text(user_password: &str) -> Result<String> {
  let salt = b"sankar_boro";
  let config = Config::default();
  let x = argon2::hash_encoded(user_password.as_bytes(), salt, &config)?;
  Ok(x)
}

pub fn encrypt_text_bytes(user_password: &Vec<u8>) -> Result<String> {
  let salt = b"sankar_boro";
  let config = Config::default();
  let x = argon2::hash_encoded(user_password, salt, &config)?;
  Ok(x)
}

pub fn validate_user_credentials(req_pass: &str, db_pass: &[u8]) -> Result<(), anyhow::Error> {
    let salt = b"sankar_boro";
    let config = Config::default();
    let req_pass = req_pass.as_bytes();
    let data = argon2::hash_encoded(req_pass, salt, &config)?;
    if data.as_bytes() != db_pass {
      return Err(anyhow::Error::msg("password mismatch"));
    }
    Ok(())
}