use actix_web::{HttpResponse, web};
use validator::Validate;
use super::schema::users;
use loony_service::{LoonyError};
use serde::Deserialize;
use super::db;
use crate::connection::conn;
use crate::App;


#[derive(Deserialize, Debug, Insertable, Validate, Clone, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
  id: i32,
  name: Option<String>,
  #[validate(email)]
  email: Option<String>,
  #[validate(length(min = 6))]
  phone: Option<String>,
  uname: Option<String>,
}

pub fn run(u_id: i32, request: &web::Form<UpdateUser>, app_data: &web::Data<App>) -> Result<(), LoonyError> {
  let con = conn(&app_data)?;
  db::update_row(u_id, &request, &con)?;
  Ok(())
}

pub async fn update_user(info: web::Path<i32>, request: web::Form<UpdateUser>, app_data: web::Data<App>) -> HttpResponse {
  if let Err(error) = request.validate() {
    HttpResponse::Ok().json(error);
  }
  match run(info.0, &request, &app_data) {
    Ok(_) => HttpResponse::Ok().body("Updated"), 
    Err(err) => HttpResponse::Ok().json(err)
  }
}