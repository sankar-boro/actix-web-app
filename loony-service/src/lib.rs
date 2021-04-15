mod db;
mod error;

use argon2::{self, Config};
pub use db::{DBConnection, RedisConnection, MysqlPool, PGPool, PGPooledConnection};
use redis::Client;

#[derive(Clone)]
pub struct Loony {
  pub db: PGPool,
  pub redis_db: Client,
}

impl Loony {
  pub fn new() -> Self {
    let db = DBConnection::connect_pg();
    let redis_db = RedisConnection::new();
    Self {
      db,
      redis_db,
    }
  }
}

pub fn encrypt_text(user_password: &str) -> String {
  let salt = b"sankar_boro";
  let config = Config::default();
  argon2::hash_encoded(user_password.as_bytes(), salt, &config).unwrap()
}


pub use error::{LoonyError, LoonyResponse};
