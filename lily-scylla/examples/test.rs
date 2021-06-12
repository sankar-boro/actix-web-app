//! Example of cookie based session
//! Session data is stored in cookie, it is limited to 4kb
//!
//! [Redis session example](https://github.com/actix/examples/tree/master/redis-session)
//!
//! [User guide](https://actix.rs/docs/middleware/#user-sessions)

use actix_session::{CookieSession, Session};
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpServer, Result};
use actix_redis::RedisSession;

/// simple index handler with session
async fn index(session: Session, req: HttpRequest) -> Result<&'static str> {
    session.insert("user", "sankar")?;
    session.renew();
    Ok("welcome!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();
    println!("Starting http server: 127.0.0.1:8000");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(Logger::default())
            .wrap(RedisSession::new("127.0.0.1:6379", &[0; 32]))
            // cookie session middleware
            // .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}