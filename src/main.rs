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
mod db;
mod builder;

use std::env;
use anyhow::Result;
use actix_cors::Cors;
use error::Error as AppError;
use actix_web::{web, cookie, App as ActixApp, HttpServer};

use time::Duration;
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware, config::PersistentSession};

pub use builder::Connections;

async fn start_server(app: Connections) -> Result<()> {
    let lp_host = env::var("LP_HOST").unwrap();
    let lp_port = env::var("LP_PORT").unwrap();
    let lp_port: u16 = lp_port.parse().unwrap();
    let pkey = env::var("PRIVATE_KEY").unwrap();
    let redis_uri = env::var("REDIS_URI").unwrap();

    let private_key = cookie::Key::from(pkey.as_bytes());

    HttpServer::new(move || {
        let cors = Cors::default()
              .allow_any_origin()
              .allow_any_method()
              .allow_any_header()
              .supports_credentials();

        ActixApp::new()
            .wrap(cors)
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(&redis_uri),
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
    .bind((lp_host, lp_port))?
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
    
    let session = db::get_scylla_connection().await;
    let pool = db::get_pg_connection().await;
    
    let app = Connections::new(session, pool);
    start_server(app).await.unwrap();
}