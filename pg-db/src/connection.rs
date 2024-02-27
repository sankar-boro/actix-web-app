use std::env;
use tokio_postgres::NoTls;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use redis::{Client, Connection};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppConnections {
  pub db_pool: Pool,
  pub session: Arc<Mutex<Connection>>
}

#[derive(Clone)]
pub struct AppConfig {
  pub APP_HOST: String,
  pub APP_PORT: u16,
  pub POSTGRES_DBNAME: String,
  pub POSTGRES_USERNAME: String,
  pub POSTGRES_PASSWORD: String,
  pub PRIVATE_KEY: String,
  pub REDIS_URI: String,
}

pub async fn pg_connection(app_config: &AppConfig) -> AppConnections {
  let mut cfg = Config::new();
  cfg.dbname = Some(app_config.POSTGRES_DBNAME.clone());
  cfg.user = Some(app_config.POSTGRES_USERNAME.clone());
  cfg.password = Some(app_config.POSTGRES_PASSWORD.clone());
  cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
  let db_pool: Pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();


  let client = Client::open(app_config.REDIS_URI.clone()).unwrap();
  let session = client.get_connection().unwrap();
  let session = Arc::new(Mutex::new(session));

  AppConnections {
    db_pool,
    session
  }
}