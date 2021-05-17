mod route;
mod user;

use std::sync::Arc;

use anyhow::Result;
use r2d2::{ManageConnection, Pool};
use scylla::{Session, SessionBuilder};
use actix_web::{App as ActixApp, HttpServer};
// use actix_redis::RedisSession;

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
}

async fn session() -> Session {
    let uri = "127.0.0.1:9042";
    SessionBuilder::new().known_node(uri).build().await.unwrap()
}

struct ScyllaConnectionManager{
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
        ActixApp::new()
        // .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            .data(app.clone())
            .configure(route::routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;
    Ok(())
}