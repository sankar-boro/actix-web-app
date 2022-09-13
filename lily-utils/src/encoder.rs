use argon2::{self, Config};
use anyhow::Result;

static SALT: &[u8; 11] = b"sankar_boro";

fn encode(u: &[u8]) -> Result<String> {
  let config = Config::default();
  Ok(argon2::hash_encoded(u, SALT, &config)?)
}

pub fn encrypt_text(user_password: &str) -> Result<String> {
  let b = user_password.as_bytes();
  encode(b)
}

pub fn encrypt_text_bytes(user_password: &Vec<u8>) -> Result<String> {
  encode(user_password)
}

pub fn validate_user_credentials(req_pass: &str, db_pass: &[u8]) -> Result<(), anyhow::Error> {
    let req_pass = req_pass.as_bytes();
    let data = encode(req_pass)?;
    if data.as_bytes() != db_pass {
      return Err(anyhow::Error::msg("WRONG_PASSWORD"));
    }
    Ok(())
}