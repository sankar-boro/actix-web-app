use actix_web::{HttpResponse, web};
use super::db;
use crate::connection::conn;
use crate::App;

pub async fn delete_one(post_id: web::Path<i32>, app_data: web::Data<App>) -> HttpResponse {
  match conn(&app_data) {
    Ok(con) => {
      match db::delete_one(post_id.0, &con) {
        Ok(_) => HttpResponse::Ok().body("Post deleted."), 
        Err(err) => HttpResponse::Ok().json(err)
      }
    },
    Err(e) => HttpResponse::Ok().json(e)
  }
}