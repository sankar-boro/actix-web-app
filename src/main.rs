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
use crate::utils::ConnectionResult;
use scylla::{
    Session, 
    SessionBuilder, 
    transport::errors::NewSessionError
};
use actix_web::{App as ActixApp, HttpServer};
use r2d2::{ManageConnection, Pool, PooledConnection};
use actix_web::web;
use actix_cors::Cors;
use log::{error};

#[derive(Clone)]
pub struct App {
    session: Pool<ScyllaConnectionManager>
}

impl App {
    fn new(session: Pool<ScyllaConnectionManager>) -> Self {
        Self {
            session,
        }
    }

    pub fn conn(&self) -> Result<PooledConnection<ScyllaConnectionManager>, r2d2::Error> {
        Ok(self.session.get()?)
    }
}

async fn session() -> Result<Session, NewSessionError> {
    let uri = "127.0.0.1:9042";
    let a = SessionBuilder::new().known_node(uri).build().await;
    a
}

pub struct ScyllaConnectionManager {
    session: Arc<Session>,
}

impl ScyllaConnectionManager {
    async fn new(s: Session) -> Self {
        Self {
            session:Arc::new(s)
        }
    }
}

impl ManageConnection for ScyllaConnectionManager {
    type Connection = Arc<Session>;

    type Error = std::io::Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Ok(self.session.clone())
    }

    fn is_valid(&self, _: &mut Self::Connection) -> Result<(), Self::Error> {
        Ok(())
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        true
    }
}

async fn builder(s: Session) -> Pool<ScyllaConnectionManager> {
    let m = ScyllaConnectionManager::new(s).await;
    let p = r2d2::Pool::builder()
        .max_size(3)
        .build(m)
        .unwrap();
    p
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
    let session = session().await;

    if let Ok(session) = session {
        let session = builder(session).await;
        let app = App::new(session);
    
        return start_server(app).await;
    }

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
    }

    Ok(())
}

impl ConnectionResult for web::Data<App> {

	fn conn_result(&self) -> Result<PooledConnection<ScyllaConnectionManager>, actix_web::Error> {
		self.as_ref()
		.conn()
		.map_err(|err| {
			AppError::from(err).into()
		})
	}
}


#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    start_scylla_app().await.unwrap();
}