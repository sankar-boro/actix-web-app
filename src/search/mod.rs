pub mod search;
use actix_web::{HttpResponse, web};
use self::search::{SearchHandler};

pub async fn search_fn(search: web::Data<SearchHandler>, query: web::Path<String>) -> Result<HttpResponse, crate::AppError> {
    let a = search.search(&query).unwrap();
    Ok(HttpResponse::Ok().json(a))
}