use crate::{user, book};
use crate::auth::login;
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/login", web::post().to(login::login));
  config.route("/get_user", web::get().to(user::get_user));

  config.service(
    web::scope("/book")
    .route("/get_all_book_nodes", web::get().to(book::get_all_book_nodes))
  );

}