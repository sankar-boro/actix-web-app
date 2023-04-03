#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
mod user;
mod auth;
mod book;
mod blog;
mod route;
mod error;
mod query;
mod utils;
mod common;
mod helpers;
mod middleware;

use std::env;
use std::sync::Arc;
use anyhow::Result;
use actix_cors::Cors;
use scylla::batch::Batch;
use error::Error as AppError;
use scylla::{ Session, SessionBuilder};
use actix_web::{web, cookie, App as ActixApp, HttpServer};

use time::Duration;
use scylla::query::Query;
use tokio_postgres::NoTls;
use scylla::frame::value::ValueList;
use scylla::frame::value::BatchValues;
use scylla::{QueryResult, BatchResult};
use scylla::transport::errors::QueryError;
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware, config::PersistentSession};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};

#[derive(Clone)]
pub struct App {
    pub session: Arc<Session>,
    pub pool: Pool,
}

impl App {
    fn new(session: Session, pool: Pool) -> Self {
        Self {
            session: Arc::new(session),
            pool,
        }
    }

    pub async fn query(&self, query: impl Into<Query>, values: impl ValueList) -> Result<QueryResult, QueryError>{
        self.session.query(query, values).await
    }

    pub async fn query_paged(&self, query: impl Into<Query>, values: impl ValueList, page: Vec<u8>) -> Result<QueryResult, QueryError>{
        let pagedata = Some(scylla::Bytes::from(page));
        self.session.query_paged(query, values, pagedata).await
    }

    pub async fn batch(&self, query: &Batch, values: impl BatchValues) -> Result<BatchResult, QueryError>{
        self.session.batch(query, values).await
    }
}

async fn start_server(app: App) -> Result<()> {
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap();
    let private_key = cookie::Key::from("authUser".as_bytes());

    HttpServer::new(move || {
        let cors = Cors::default()
              .allow_any_origin()
              .allow_any_method()
              .allow_any_header()
              .supports_credentials();

        ActixApp::new()
            .wrap(cors)
            .wrap(
                // RedisSession::new("127.0.0.1:6379", &[0; 32])
                // .cookie_name("lily-session")
                // .cookie_http_only(true)
                // .ttl(86400)
                SessionMiddleware::builder(
                    RedisActorSessionStore::new("127.0.0.1:6379"),
                    private_key.clone(),
                )
                .session_lifecycle(
                    PersistentSession::default()
                        .session_ttl(Duration::days(5))
                )
                .build()
            )
            .app_data(web::Data::new(app.clone()))
            .configure(route::routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await?;
    Ok(())
}

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let uri = "127.0.0.1:9042";
    let session = SessionBuilder::new().known_node(uri).build().await.unwrap();
    let mut cfg = Config::new();
    cfg.dbname = Some("sankar".to_string());
    cfg.user = Some("sankar".to_string());
    cfg.password = Some("sankar".to_string());
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pool: Pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let app = App::new(session, pool);
    start_server(app).await.unwrap();
}