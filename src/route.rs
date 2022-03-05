use crate::user;
use crate::book;
use crate::node;

use actix_web::{web, HttpResponse};
use crate::middleware::Authentication;

async fn home() -> HttpResponse {
  HttpResponse::Ok().body("Home!")
}

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/", web::get().to(home));
  config.route("/login", web::post().to(user::login));
  config.route("/signup", web::post().to(user::signup));
  config.service(
    web::scope("/user")
    .route("/session", web::get().to(user::user_session))
  );
  config.service(web::resource("/upload/image").route(web::post().to(book::upload_image)));
  config.route("/users", web::post().to(user::users));
  config.service(
    web::scope("/user")
    .wrap(Authentication{})
    .route("/get/{userId}", web::get().to(user::get))
    .route("/update", web::post().to(user::update))
  );
  config.route("/books", web::get().to(book::getAllBooks));
  config.service(
    web::scope("/book")
    .wrap(Authentication{})
    .route("/get/{bookId}", web::get().to(book::getAllNodesFromBookId))
    .route("/create", web::post().to(book::create))
    .route("/delete", web::post().to(book::delete))
    .route("/update", web::post().to(book::update))
  );
  config.service(
    web::scope("/node")
    .wrap(Authentication{})
    .route("/create", web::post().to(node::create))
    .route("/merge", web::post().to(node::merge))
    .route("/delete", web::post().to(node::delete))
    .route("/delete/update", web::post().to(node::deleteAndUpdate))
    .route("/update", web::post().to(node::update))
  );
  config.service(
    web::scope("")
    .wrap(Authentication{})
    .route("/logout", web::post().to(user::logout_user))
  );
}