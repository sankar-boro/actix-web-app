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
    .route("/get", web::get().to(user::read_rows))
    .route("/get/{id}", web::get().to(user::read_row_by_id))
    .route("/update/{id}", web::post().to(user::update_user))
  );
  config.service(
    web::scope("/post")
    .wrap(AuthService{})
    .route("/all", web::get().to(post::read_rows))
    .route("/get/{id}", web::get().to(post::read_row_by_id))
    .route("/create", web::post().to(post::create))
    .route("/update/{id}", web::post().to(post::update_post))
    .route("/delete/{id}", web::post().to(post::delete_post))
  );
  config.service(
    web::scope("")
    .wrap(AuthService{})
    .route("/logout/{id}", web::post().to(user::logout_user))
  );
}
