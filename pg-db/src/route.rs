use crate::{user, book, blog};
use crate::auth::login;
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/login", web::post().to(login::login));
  config.route("/get_user", web::get().to(user::get_user));

  // user
  config.service(
    web::scope("/user")
    .route("/session", web::get().to(user::user_session))
  );

  config.route("/books", web::get().to(book::get_all_books));
  config.service(
    web::scope("/book")
    .route("/get_all_book_nodes", web::get().to(book::get_all_book_nodes))
  );

  config.route("/blogs", web::get().to(blog::get_all_blogs));
  config.service(
    web::scope("/blog")
    .route("/get_all_blog_nodes", web::get().to(blog::get_all_blog_nodes))
  );

}