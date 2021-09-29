use actix_session::{Session};
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpServer, Result};
use actix_redis::RedisSession;

async fn index(session: Session, _: HttpRequest) -> Result<&'static str> {
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