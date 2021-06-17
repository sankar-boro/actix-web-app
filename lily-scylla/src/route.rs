use crate::user;
use crate::post;

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
    web::scope("/post")
    .wrap(Authentication{})
    .route("/all", web::get().to(post::get_all))
    .route("/get/{id}", web::get().to(post::get_one))
    .route("/create", web::post().to(post::create_one))
    .route("/update/{id}", web::post().to(post::update_one))
    // .route("/delete/{id}", web::post().to(post::delete_one))
  );
  // config.service(
  //   web::scope("/upload")
  //   .wrap(Authentication{})
  //   .route("/image", web::post().to(post::upload_image))
  // );
  config.service(web::resource("/upload/image").route(web::post().to(post::upload_image)));
  config.service(
    web::scope("")
    .wrap(Authentication{})
    .route("/logout", web::post().to(user::logout_user))
  );
}

