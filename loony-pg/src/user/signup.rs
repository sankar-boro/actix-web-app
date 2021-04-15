use serde::{Deserialize};
use validator::Validate;
use super::{db::ReadRow, schema::users};
use actix_web::{web, HttpResponse};
use loony_service::{LoonyError, encrypt_text};
use super::db;
use crate::connection::conn;
use crate::App;

trait Row {
  fn into_row(&self) -> Self;
}

#[derive(Deserialize, Debug, Insertable, Validate, Clone)]
#[table_name = "users"]
pub struct SignupFormData {
  name: String,
  #[validate(email)]
  email: String,
  #[validate(length(min = 6))]
  password: String,
}

impl Row for SignupFormData {
  fn into_row(&self) -> Self {
    let password = encrypt_text(&self.password);
    SignupFormData {
      name: self.name.clone(),
      email: self.email.clone(),
      password,
    }
  }
}

pub fn run(request: &web::Form<SignupFormData>, app_data: &web::Data<App>) -> Result<ReadRow, LoonyError> {
  let con = conn(&app_data)?;
  Ok(db::insert(&request.into_row(), &con)?)
}

pub async fn sign_up(request: web::Form<SignupFormData>, app_data: web::Data<App>) -> HttpResponse {
  if let Err(error) = request.validate() {
    HttpResponse::Ok().json(error);
  }
  match run(&request, &app_data) {
    Ok(data) => HttpResponse::Ok().json(data), 
    Err(err) => HttpResponse::Ok().json(err)
  }
}