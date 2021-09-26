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
    .route("/all", web::get().to(book::get_all))
    .route("/get/{id}", web::get().to(book::get_one))
    .route("/getall/{id}", web::get().to(book::get_all_from_id))
    .route("/create/new/book", web::post().to(book::create_new_book))
    .route("/create/new/page", web::post().to(book::create_new_page))
    .route("/create/new/section", web::post().to(book::create_new_section))
    .route("/create/new/chapter", web::post().to(book::create_new_chapter))
    .route("/create/update/chapter", web::post().to(book::create_and_update_chapter))
    .route("/update", web::post().to(book::update_one))
    .route("/delete/sub_section/last", web::post().to(book::delete_section_last))
    .route("/delete/sub_section/first", web::post().to(book::delete_section_first))
    .route("/delete/main_section", web::post().to(book::delete_main_section))
  );
  // config.service(
  //   web::scope("/upload")
  //   .wrap(Authentication{})
  //   .route("/image", web::post().to(post::upload_image))
  // );
  config.service(web::resource("/upload/image").route(web::post().to(book::upload_image)));
  config.service(
    web::scope("")
    .wrap(Authentication{})
    .route("/logout", web::post().to(user::logout_user))
  );
}

