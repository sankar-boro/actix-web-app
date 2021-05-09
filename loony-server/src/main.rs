use std::sync::Arc;

use anyhow::Result;
use loony_scylla::{route, App};
use scylla::{Session, SessionBuilder};
use actix_web::{App as ActixApp, HttpServer};
use scylla::transport::session::{IntoTypedRows};


async fn session() -> Session {
    let uri = "127.0.0.1:9042";
    SessionBuilder::new().known_node(uri).build().await.unwrap()
}

#[actix_web::main]
async fn main() -> Result<()> {
    let session = session().await;
    let app = App::new(Arc::new(session));

    HttpServer::new(move || {
        ActixApp::new()
            .data(app.clone())
            .configure(route::routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;
    Ok(())
}