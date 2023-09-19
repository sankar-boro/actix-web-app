use crate::{user, book, blog, booknode};
use crate::auth::login;
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {

  // auth
  config.route("/login", web::post().to(login::login));
  config.route("/get_user", web::get().to(user::get_user));

  // user
  config.service(
    web::scope("/user")
    .route("/session", web::get().to(user::user_session))
  );

  // books
  config.route("/books", web::get().to(book::get_all_books));

  // book
  config.service(
    web::scope("/book")
    .route("/node/{bookid}/all", web::get().to(book::node_all))
    .route("/title/{bookid}/all", web::get().to(book::title_all))
  );

  // booknode
  config.service(
    web::scope("/booknode")
    .route("/pages/{docid}/{pageid}", web::get().to(booknode::nodes))
  );

  // blogs
  config.route("/blogs", web::get().to(blog::get_all_blogs));
  
  // blog
  config.service(
    web::scope("/blog")
    .route("/get_all_blog_nodes", web::get().to(blog::get_all_blog_nodes))
  );

}