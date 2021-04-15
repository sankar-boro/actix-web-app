use super::db;
use crate::connection::conn;
use actix_web::{web, HttpResponse};
use crate::App;

pub async fn get_all(app_data: web::Data<App>) -> HttpResponse {
  match conn(&app_data) {
    Ok(con) => {
      match db::get_all(&con) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::Ok().json(err)
      }
    },
    Err(e) => HttpResponse::Ok().json(e),
  }
}

pub async fn get_one(post_id: web::Path<i32>, app_data: web::Data<App>) -> HttpResponse {
  match conn(&app_data) {
    Ok(con) => {
      match db::get_one(post_id.0, &con) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::Ok().json(err)
      }
    },
    Err(e) => HttpResponse::Ok().json(e),
  }
}