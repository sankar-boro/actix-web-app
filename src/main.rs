#[macro_use]
extern crate log;

use std::env;
use anyhow::Result;
use actix_cors::Cors;
use actix_web::middleware::Condition;
use pg_db::{pg_connection, route};
use actix_web::{web, cookie, App as ActixApp, HttpServer};
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware, config::PersistentSession};
use time::Duration;

async fn start_server<T: Clone + Send + 'static>(app: T) -> Result<()> {
    
    let lp_host = env::var("LP_HOST").unwrap();
    let lp_port = env::var("LP_PORT").unwrap();
    let lp_port: u16 = lp_port.parse().unwrap();
    let pkey = env::var("PRIVATE_KEY").unwrap();
    let redis_uri = env::var("REDIS_URI").unwrap();
    let dev = env::var("DEV").unwrap();
    let dev: bool = match dev.as_str() {
        "TRUE" => true,
        _ => false
    };

    info!("starting server at host: {}, port: {}", lp_host, lp_port);

    let private_key = cookie::Key::from(pkey.as_bytes());
    HttpServer::new(move || {

        ActixApp::new()
            .wrap(Condition::new(dev, Cors::permissive()))
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
    let conn = pg_connection().await;
    start_server(conn).await.unwrap();
}