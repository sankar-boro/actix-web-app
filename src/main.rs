#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
mod route;
mod user;
mod helpers;
mod middleware;
mod utils;
mod error;
mod query;
mod auth;
mod book;
mod blog;
mod common;
// mod search;

use std::env;
use std::sync::Arc;
use anyhow::Result;
// use async_std::sync::Mutex;
use error::Error as AppError;
use actix_redis::RedisSession;
use scylla::batch::Batch;
use scylla::{
    Session, 
    SessionBuilder
};
use actix_web::{App as ActixApp, HttpServer};
use actix_web::web::{
    self, 
    // Data
};
use actix_cors::Cors;
// use log::{error};

use scylla::{QueryResult, BatchResult};
use scylla::query::Query;
use scylla::frame::value::ValueList;
use scylla::frame::value::BatchValues;
use scylla::transport::errors::QueryError;
// use search::search::SearchHandler;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

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

// fn search_data() {
//     // let search = Data::new(Mutex::new(Search::new()));
//     let (search, index) = SearchHandler::new();
//     let search = Data::new(search);
//     let index = Data::new(Mutex::new(index));
// }

async fn start_server(app: App) -> Result<()> {
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
              .allow_any_origin()
              .allow_any_method()
              .allow_any_header()
              .supports_credentials();

        ActixApp::new()
            .wrap(cors)
            .wrap(
                RedisSession::new("127.0.0.1:6379", &[0; 32])
                .cookie_name("lily-session")
                .cookie_http_only(true)
                .ttl(86400)
            )
            .app_data(web::Data::new(app.clone()))
            // .app_data(Data::clone(&search))
            // .app_data(index.clone())
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