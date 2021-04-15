use diesel::pg::PgConnection;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use r2d2::{self, Pool};
use dotenv::dotenv;
use std::env;
use redis::Client;

pub type MysqlPool = r2d2::Pool<ConnectionManager<diesel::MysqlConnection>>;
pub type PGPool = r2d2::Pool<ConnectionManager<diesel::pg::PgConnection>>;
pub type PGPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

static POOL_SIZE: u32 = 8;

pub struct DBConnection(String);

impl DBConnection {

  pub fn connect_pg() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
            .max_size(POOL_SIZE)
            .build(manager)
            .unwrap()
  }
}

pub struct RedisConnection;

impl RedisConnection {
  pub fn new() -> Client {
    Client::open("redis://127.0.0.1/").unwrap()
  }
}