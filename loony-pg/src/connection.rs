use actix_web::web;
use loony_service::{LoonyError};

use diesel::pg::PgConnection;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use dotenv::dotenv;
use std::env;
use crate::App;

pub type PGPool = r2d2::Pool<ConnectionManager<diesel::pg::PgConnection>>;
pub type PGPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn conn(app_data: &web::Data<App>) -> Result<PGPooledConnection, LoonyError> {
  Ok(app_data.conn.get()?)
}


static POOL_SIZE: u32 = 8;

pub struct DBConnection(String);

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