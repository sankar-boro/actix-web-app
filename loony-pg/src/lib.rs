#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;

use actix_web::{App as ActixApp, HttpServer};
mod route;
mod post;
mod user;
mod connection;
use actix_cors::Cors;
use actix_redis::RedisSession;
use connection::{PGPool, DBConnection};

#[derive(Clone)]
pub struct App {
    conn: PGPool,
}

impl App {
    fn new() -> Self {
        Self {
            conn: DBConnection::connect_pg(),
        }
    }
}

pub async fn start_pg_app() -> std::io::Result<()> {
    env_logger::init();
    info!("INITIALIZING APP");
        std::fs::create_dir_all("./tmp").unwrap();


    let app = App::new();
    HttpServer::new(move || {
        ActixApp::new()
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            .wrap(Cors::permissive())
            // .service(web::resource("/create_post")
            //     .route(web::get().to(home))
            //     .route(web::post().to(create_post)),)
            .data(app.clone())
            .configure(route::routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}