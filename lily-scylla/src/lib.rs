mod route;
mod user;
mod helpers;
mod middleware;
mod book;
mod utils;
mod error;
mod query;

use std::sync::Arc;

use actix_session::CookieSession;
use anyhow::Result;
use error::Error as AppError;
use actix_redis::RedisSession;
use crate::utils::ConnectionResult;
use scylla::{Session, SessionBuilder};
use helpers::error::Error as RequestError;
use actix_web::{App as ActixApp, HttpServer, http};
use r2d2::{ManageConnection, Pool, PooledConnection};
use actix_web::web;
use actix_cors::Cors;
use time::Duration;

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

async fn session() -> Session {
    let uri = "127.0.0.1:9042";
    SessionBuilder::new().known_node(uri).build().await.unwrap()
}

pub struct ScyllaConnectionManager {
    session: Arc<Session>,
}

impl ScyllaConnectionManager {
    async fn new() -> Self {
        Self {
            session:Arc::new(session().await)
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

async fn builder() -> Pool<ScyllaConnectionManager> {
    let m = ScyllaConnectionManager::new().await;
    let p = r2d2::Pool::builder()
        .max_size(3)
        .build(m)
        .unwrap();
    p
}

pub async fn start_scylla_app() -> Result<()> {
    let session = builder().await;
    let app = App::new(session);

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
                .cookie_max_age(Some(Duration::days(1)))
            )
            .data(app.clone())
            .configure(route::routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;
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