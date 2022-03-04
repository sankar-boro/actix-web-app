use crate::user;
use crate::book;

use actix_web::{web, HttpResponse};
use crate::middleware::Authentication;

async fn home() -> HttpResponse {
  HttpResponse::Ok().body("Home!")
}

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/", web::get().to(home));
  config.route("/login", web::post().to(user::login));
  config.route("/signup", web::post().to(user::create_user));
  config.service(
    web::scope("/user")
    .route("/session", web::get().to(user::user_session))
  );
  config.service(web::resource("/upload/image").route(web::post().to(book::upload_image)));
  config.service(
    web::scope("/user")
    .wrap(Authentication{})
    .route("/get/all", web::get().to(user::get_all))
    .route("/get/authuser", web::get().to(user::get_one))
    .route("/update/authuser", web::post().to(user::update_one))
  );
  config.service(
    web::scope("/book")
    .wrap(Authentication{})
    .route("/getAllBooks", web::get().to(book::getAllBooks))
    .route("/getAllNodesFromBookId/{bookId}", web::get().to(book::getAllNodesFromBookId))
    .route("/createNewBook", web::post().to(book::createNewBook))
    .route("/appendNode", web::post().to(book::append_node))
    .route("/mergeNode", web::post().to(book::merge_node))
    .route("/deleteBook/{bookId}", web::post().to(book::deleteBook))
    .route("/deleteLastNode", web::post().to(book::deleteLastNode))
    .route("/updateBotNodeOnDeleteNode", web::post().to(book::updateBotNodeOnDeleteNode))
  );
  config.service(
    web::scope("")
    .wrap(Authentication{})
    .route("/logout", web::post().to(user::logout_user))
  );
}