use actix_web::{HttpResponse, web};
use loony_service::{LoonyError};
use super::db;
use crate::connection::conn;
use crate::App;

pub fn run(u_id: i32, app_data: &web::Data<App>) -> Result<(), LoonyError> {
  let con = conn(&app_data)?;
  db::delete_row(u_id, &con)?;
  Ok(())
}

pub async fn delete_post(info: web::Path<i32>, app_data: web::Data<App>) -> HttpResponse {
  match run(info.0, &app_data) {
    Ok(_) => HttpResponse::Ok().body("Post deleted."), 
    Err(err) => HttpResponse::Ok().json(err)
  }
}