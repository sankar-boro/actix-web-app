use crate::user;
use crate::book;
use crate::blog;
use crate::booknode;
use crate::blognode;
// use crate::search;

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
  
  // #search
  // config.route("/search/{query}", web::get().to(search::search_fn));

  config.service(
    web::scope("/user")
    .wrap(Authentication{})
    .route("/get/{userId}", web::get().to(user::get))
    .route("/update", web::post().to(user::update))
  );
  //
  config.route("/books", web::get().to(book::getAllBooks));
  config.service(
    web::scope("/author/books")
    // .wrap(Authentication{})
    .route("/get/{author_id}", web::get().to(book::getAllNodesFromAuthorId))
  );
  config.service(
    web::scope("/book")
    .wrap(Authentication{})
    .route("/get/{bookId}", web::get().to(book::getAllNodesFromBookId))
    .route("/create", web::post().to(book::create))
    .route("/create/new_session", web::post().to(book::create_book_sessionv2))
    .route("/delete/{deleteId}", web::post().to(book::delete))
    .route("/update", web::post().to(book::update))
  );
  config.service(
    web::scope("/booknode")
    .wrap(Authentication{})
    .route("/create", web::post().to(booknode::create))
    .route("/merge", web::post().to(booknode::merge))
    .route("/delete", web::post().to(booknode::delete))
    .route("/delete/update", web::post().to(booknode::deleteAndUpdate))
    .route("/update", web::post().to(booknode::update))
  );
  //
  config.route("/blogs", web::get().to(blog::getAllBlogs));
  config.service(
    web::scope("/blog")
    .wrap(Authentication{})
    .route("/get/{blogId}", web::get().to(blog::getAllNodesFromBlogId))
    .route("/create", web::post().to(blog::create))
    .route("/delete/{deleteId}", web::post().to(blog::delete))
    .route("/update", web::post().to(blog::update))
  );
  config.service(
    web::scope("/blognode")
    .wrap(Authentication{})
    .route("/create", web::post().to(blognode::create))
    .route("/merge", web::post().to(blognode::merge))
    .route("/delete", web::post().to(blognode::delete))
    .route("/delete/update", web::post().to(blognode::deleteAndUpdate))
    .route("/update", web::post().to(blognode::update))
  );
  //
  config.service(
    web::scope("")
    .wrap(Authentication{})
    .route("/logout", web::post().to(user::logout_user))
  );

}