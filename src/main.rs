#[macro_use]
extern crate log;

use std::{env, sync::{Arc, Mutex}};
use anyhow::Result;
use actix_cors::Cors;
use actix_web::middleware::Condition;
use pg_db::{pg_connection, route};
use actix_web::{web, cookie, App as ActixApp, HttpServer};
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware, config::PersistentSession};
use time::Duration;
use actix_web::http::header;
use pg_db::{AppConfig, AppConnections};

async fn start_server<T: Clone + Send + 'static>(app: T, app_config: &AppConfig) -> Result<()> {
    let server = HttpServer::new(move || {
        let cors: Cors = Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .supports_credentials()
        .max_age(3600);
        ActixApp::new()
            .wrap(cors)
            .app_data(web::Data::new(app.clone()))
            .configure(route::routes)
    })
    .bind((app_config.APP_HOST.clone(), app_config.APP_PORT))?;
    let address = server.addrs();
    info!("Listening to address: {:?}", address);
    server.run().await?;
    Ok(())
}

#[actix_web::main]
async fn main() {
    // std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    // std::env::set_var("RUST_LOG", "info");
    // std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let APP_HOST = env::var("APP_HOST").unwrap();
    let APP_PORT = env::var("APP_PORT").unwrap();
    let APP_PORT: u16 = APP_PORT.parse().unwrap();
    let POSTGRES_DBNAME = env::var("POSTGRES_DBNAME").unwrap();
    let POSTGRES_USERNAME = env::var("POSTGRES_USERNAME").unwrap();
    let POSTGRES_PASSWORD = env::var("POSTGRES_PASSWORD").unwrap();
    let PRIVATE_KEY = env::var("PRIVATE_KEY").unwrap();
    let REDIS_URI = env::var("REDIS_URI").unwrap();

    let app_config = AppConfig {
        APP_HOST,
        APP_PORT,
        POSTGRES_DBNAME,
        POSTGRES_USERNAME,
        POSTGRES_PASSWORD,
        PRIVATE_KEY: PRIVATE_KEY.clone(),
        REDIS_URI,
    };

    
    let private_key = cookie::Key::from(&PRIVATE_KEY.as_bytes());

    let conn = pg_connection(&app_config).await;
    start_server(conn, &app_config).await.unwrap();
}