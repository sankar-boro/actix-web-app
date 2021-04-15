use actix_web::{HttpResponse, web};
use validator::Validate;
use super::schema::posts;
use loony_service::{LoonyError};
use serde::Deserialize;
use super::db;
use crate::connection::conn;
use crate::App;

#[derive(Deserialize, Debug, Insertable, Validate, Clone, AsChangeset)]
#[table_name = "posts"]
pub struct UpdatePost {
  id: i32,
  title: Option<String>,
  body: Option<String>
}

pub fn run(u_id: i32, request: &web::Form<UpdatePost>, app_data: &web::Data<App>) -> Result<(), LoonyError> {
  let con = conn(&app_data)?;
  db::update_row(u_id, &request, &con)?;
  Ok(())
}

pub async fn update_post(info: web::Path<i32>, request: web::Form<UpdatePost>, app_data: web::Data<App>) -> HttpResponse {
  if let Err(error) = request.validate() {
    HttpResponse::Ok().json(error);
  }
  match run(info.0, &request, &app_data) {
    Ok(_) => HttpResponse::Ok().body("Updated"), 
    Err(err) => HttpResponse::Ok().json(err)
  }
}