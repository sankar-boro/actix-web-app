use crate::{user, post};
use actix_web::{web, HttpResponse};
use loony_middleware::authentication::AuthService;

async fn home() -> HttpResponse {
  HttpResponse::Ok().body("Home!")
}

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/", web::get().to(home));
  config
  .route("/login", web::post().to(user::login))
  .route("/signup", web::post().to(user::sign_up));
  config.service(
    web::scope("/user")
    .wrap(AuthService{})
    .route("/get", web::get().to(user::get_all))
    .route("/get/{id}", web::get().to(user::get_one))
    .route("/update/{id}", web::post().to(user::update_one))
  );
  config.service(
    web::scope("/post")
    .wrap(AuthService{})
    .route("/all", web::get().to(post::get_all))
    .route("/get/{id}", web::get().to(post::get_one))
    .route("/create", web::post().to(post::create_one))
    .route("/update/{id}", web::post().to(post::update_one))
    .route("/delete/{id}", web::post().to(post::delete_one))
  );
  config.service(
    web::scope("")
    .wrap(AuthService{})
    .route("/logout/{id}", web::post().to(user::logout_user))
  );
}
