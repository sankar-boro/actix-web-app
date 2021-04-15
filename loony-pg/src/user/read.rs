use serde::{Serialize, Deserialize};
use validator::{Validate};
use actix_web::{web, HttpResponse};
use loony_service::{LoonyError};
use super::{db, db::ReadRow};
use crate::connection::conn;
use chrono::{NaiveDateTime};
use crate::App;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  id: i32,
  email: String,
  exp: i64,
  iat: i64,
}
#[derive(Deserialize, Debug, Validate)]
pub struct LoginUserInfo {
  #[validate(email)]
  email: String,
  password: String,
}

#[derive(Queryable, Serialize, Debug)]
pub struct UserInfo {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub token: String,
}

fn run_read_rows(app_data: &web::Data<App>) -> Result<Vec<ReadRow>, LoonyError> {
  let con = conn(&app_data)?;
  Ok(db::read_rows(&con)?)
}

pub async fn read_rows(app_data: web::Data<App>) -> HttpResponse {
  match run_read_rows(&app_data) {
    Ok(res) => HttpResponse::Ok().json(res),
    Err(err) => HttpResponse::Ok().json(err)
  }
}

fn run_read_row_by_id(u_id: i32, app_data: &web::Data<App>) -> Result<ReadRow, LoonyError> {
  let con = conn(&app_data)?;
  Ok(db::read_row_by_id(u_id, &con)?)
}

pub async fn read_row_by_id(info: web::Path<i32>, app_data: web::Data<App>) -> HttpResponse {
  let user_id = info.0;
  match run_read_row_by_id(user_id, &app_data) {
    Ok(res) => HttpResponse::Ok().json(res),
    Err(err) => HttpResponse::Ok().json(err)
  }
}