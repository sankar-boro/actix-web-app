use std::env;
use dotenv::dotenv;
use actix_web::web;
use diesel::{
  pg::PgConnection,
  r2d2::{
    ConnectionManager,
    PooledConnection
  }
};

use crate::App;
use loony_service::{LoonyError};

pub type PGPool = r2d2::Pool<ConnectionManager<diesel::pg::PgConnection>>;
pub type PGPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn conn(app_data: &web::Data<App>) -> Result<PGPooledConnection, LoonyError> {
  Ok(app_data.conn.get()?)
}

/// DBConnection 
pub struct DBConnection(String);


static POOL_SIZE: u32 = 8;
impl DBConnection {
  pub fn connect_pg() -> PGPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
            .max_size(POOL_SIZE)
            .build(manager)
            .unwrap()
  }
}