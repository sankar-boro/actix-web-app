mod route;
mod user;
mod helpers;
mod middleware;
mod book;
mod utils;
mod error;
mod query;
mod auth;

use std::sync::Arc;
use anyhow::Result;
use error::Error as AppError;
use actix_redis::RedisSession;
use scylla::{
    Session, 
    SessionBuilder, 
    transport::errors::NewSessionError
};
use actix_web::{App as ActixApp, HttpServer};
use actix_web::web;
use actix_cors::Cors;
use log::{error};

#[derive(Clone)]
pub struct App {
    session: Arc<Session>
}

impl App {
    fn new(session: Session) -> Self {
        Self {
            session: Arc::new(session),
        }
    }
}

async fn start_server(app: App) -> Result<()> {
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
            )
            .app_data(web::Data::new(app.clone()))
            .configure(route::routes)
    })
    .bind("127.0.0.1:7500")?
    .run()
    .await?;
    Ok(())
}

pub async fn start_scylla_app() -> Result<()> {
    let uri = "127.0.0.1:9042";
    let session = SessionBuilder::new().known_node(uri).build().await;
    if let Err(err) = session {
        match err {
            NewSessionError::FailedToResolveAddress(e) => error!("FailedToResolveAddress, {}", e),
            NewSessionError::EmptyKnownNodesList => error!("EmptyKnownNodesList"),
            NewSessionError::DbError(e, er) => error!("DbError, {} {}", e, er),
            NewSessionError::BadQuery(e) => error!("BadQuery, {}", e),
            NewSessionError::IoError(e) => {
                error!("IoError, {}", e);
                println!("Would you mind to check if you have started scylladb service. Command is: \"sudo systemctl start scylla-server\" ");
            },
            NewSessionError::ProtocolError(e) => error!("ProtocolError, {}", e),
            NewSessionError::InvalidMessage(e) => error!("InvalidMessage, {}", e),
            NewSessionError::TimeoutError => error!("TimeoutError"),
        }
        panic!("Could not start server");
    }
    if let Ok(session) = session {
        let app = App::new(session);
        start_server(app).await.unwrap();
    }
    Ok(())
}

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    start_scylla_app().await.unwrap();
}