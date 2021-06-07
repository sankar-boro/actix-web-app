mod error;

use argon2::{self, Config, Error};

pub fn encrypt_text(user_password: &str) -> Result<String, Error> {
  let salt = b"sankar_boro";
  let config = Config::default();
  argon2::hash_encoded(user_password.as_bytes(), salt, &config)
}

pub fn encrypt_text_bytes(user_password: &Vec<u8>) -> Result<String, Error> {
  let salt = b"sankar_boro";
  let config = Config::default();
  argon2::hash_encoded(user_password, salt, &config)
}


pub use error::{WebResponse, WebResponseError};
