use crate::user;
use actix_web::{web, HttpResponse};

async fn home() -> HttpResponse {
  HttpResponse::Ok().body("Home!")
}

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/", web::get().to(home));
  config.service(
    web::scope("/user")
    .route("/create", web::post().to(user::create_user))
    .route("/get", web::get().to(user::get_one))
    .route("/delete/{user_id}", web::post().to(user::delete_one))
  );
}

