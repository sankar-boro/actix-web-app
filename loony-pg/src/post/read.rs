use super::db::{self, ReadRow};
use loony_service::{LoonyError};
use crate::connection::conn;
use actix_web::{web, HttpResponse};
use crate::App;

fn run(app_data: &web::Data<App>) -> Result<Vec<ReadRow>, LoonyError> {
  let con = conn(&app_data)?;
  Ok(db::read_rows(&con)?)
}

pub fn read_rows(app_data: web::Data<App>) -> HttpResponse {
  match run(&app_data) {
    Ok(res) => HttpResponse::Ok().json(res),
    Err(err) => HttpResponse::Ok().json(err)
  }
}

fn run_read_row_by_id(p_id: i32, app_data: &web::Data<App>) -> Result<ReadRow, LoonyError> {
  let con = conn(&app_data)?;
  Ok(db::read_row_by_id(p_id, &con)?)
}

pub async fn read_row_by_id(info: web::Path<i32>, app_data: web::Data<App>) -> HttpResponse {
  let user_id = info.0;
  match run_read_row_by_id(user_id, &app_data) {
    Ok(res) => HttpResponse::Ok().json(res),
    Err(err) => HttpResponse::Ok().json(err)
  }
}