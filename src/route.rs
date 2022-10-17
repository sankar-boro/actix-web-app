use crate::user;
use crate::book;
use crate::blog;
use crate::booknode;
use crate::blognode;
use crate::common;
use crate::settings;
// use crate::search;

use actix_web::{web, HttpResponse};
use crate::middleware::Authentication;

async fn home() -> HttpResponse {
  HttpResponse::Ok().body("Home!")
}

pub fn routes(config: &mut web::ServiceConfig) {
  // #search
  // config.route("/search/{query}", web::get().to(search::search_fn));
  // basic
  config.route("/", web::get().to(home));
  config.route("/login", web::post().to(user::login));
  config.route("/signup", web::post().to(user::signup));

  // auth
  config.service(
    web::scope("/auth")
    .wrap(Authentication{})
    .route("/logout", web::post().to(user::logout_user))
  );

  // image
  config.service(web::resource("/upload/image").route(web::post().to(book::upload_image)));

  // user
  config.service(
    web::scope("/user")
    .route("/session", web::get().to(user::user_session))
    .wrap(Authentication{})
    .route("/get/{userId}", web::get().to(user::get))
    .route("/update", web::post().to(user::update))
    .route("/add_category", web::post().to(user::add_category))
    .route("/delete_category", web::post().to(user::delete_category))
    .route("/user_categories", web::get().to(user::get_user_categories))
  );
  config.route("/users", web::post().to(user::users));
  config.route("/all_category", web::get().to(user::get_all_category));
  
  // book
  config.route("/books", web::get().to(book::getBooksWithPageSize));
  config.route("/books/next", web::post().to(book::getNextBooksWithPageSize));
  config.service(
    web::scope("/author")
    // .wrap(Authentication{})
    .route("/books/get/{author_id}", web::get().to(book::getPagedBooksForAuthorId))
    .route("/blogs/get/{author_id}", web::get().to(book::getPagedBlogsForAuthorId))
    .route("/next_books/get/{author_id}", web::post().to(book::getNextPageBooksForAuthorId))
    .route("/next_blogs/get/{author_id}", web::post().to(book::getNextPageBlogsForAuthorId))
  );
  config.route("/create/sessionv2", web::post().to(common::create_sessionv2));
  config.service(
    web::scope("/book")
    .route("/get/{bookId}", web::get().to(book::getBookNodesWithPageSizeFromId))
    .route("/category/{category}", web::get().to(book::getBooksWithPageSizeCategories))
    .route("/next_category/{category}", web::post().to(book::getBooksWithPageSizeCategoriesNext))
    .route("/nextpage/{bookId}", web::post().to(book::getNextBookNodesWithPageSizeFromId))
    .wrap(Authentication{})
    .route("/create", web::post().to(book::create))
    .route("/delete/{deleteId}/{category}", web::post().to(book::delete))
    .route("/update", web::post().to(book::update))
    .route("/settings/create", web::post().to(settings::create))
    .route("/settings/update", web::post().to(settings::update))
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
  config.route("/blogs", web::get().to(blog::getBlogsWithPageSize));
  config.route("/blogs/next", web::post().to(blog::getNextBlogsWithPageSize));
  config.service(
    web::scope("/blog")
    .route("/get/{blogId}", web::get().to(blog::getBlogNodesWithPageSizeFromId))
    .route("/category/{category}", web::get().to(blog::getBlogsWithPageSizeCategories))
    .route("/next_category/{category}", web::post().to(blog::getBlogsWithPageSizeCategoriesNext))
    .route("/nextpage/{blogId}", web::post().to(blog::getNextBlogNodesWithPageSizeFromId))
    .wrap(Authentication{})
    .route("/create", web::post().to(blog::create))
    .route("/delete/{deleteId}/{category}", web::post().to(blog::delete))
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

}