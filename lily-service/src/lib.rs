mod error;

use argon2::{self, Config};

pub fn encrypt_text(user_password: &str) -> String {
  let salt = b"sankar_boro";
  let config = Config::default();
  argon2::hash_encoded(user_password.as_bytes(), salt, &config).unwrap()
}

pub fn encrypt_text_bytes(user_password: &Vec<u8>) -> String {
  let salt = b"sankar_boro";
  let config = Config::default();
  argon2::hash_encoded(user_password, salt, &config).unwrap()
}


pub use error::{WebResponse, WebResponseError};
