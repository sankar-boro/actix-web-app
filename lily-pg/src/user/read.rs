use serde::{Serialize, Deserialize};
use validator::{Validate};
use actix_web::{web, HttpResponse};
use crate::connection::conn;
use chrono::{NaiveDateTime};
use super::db;
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

pub async fn get_all(app_data: web::Data<App>) -> HttpResponse {
  match conn(&app_data) {
    Ok(con) => {
      match db::get_all(&con) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::Ok().json(err)
      }
    }
    Err(e) => HttpResponse::Ok().json(e)
  }
}


pub async fn get_one(user_id: web::Path<i32>, app_data: web::Data<App>) -> HttpResponse {
  match conn(&app_data) {
    Ok(con) => {
      match db::read_one(user_id.0, &con) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::Ok().json(err)
      }
    }
    Err(e) => HttpResponse::Ok().json(e)
  }
}