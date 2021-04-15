use serde::{Deserialize};
use validator::Validate;
use super::schema::users;
use actix_web::{web, HttpResponse};
use loony_service::encrypt_text;
use super::db;
use crate::connection::conn;
use crate::App;

trait EncryptedRow {
  fn enc_row(&self) -> Self;
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

impl EncryptedRow for SignupFormData {
  fn enc_row(&self) -> Self {
    let password = encrypt_text(&self.password);
    SignupFormData {
      name: self.name.clone(),
      email: self.email.clone(),
      password,
    }
  }
}

pub async fn sign_up(request: web::Form<SignupFormData>, app_data: web::Data<App>) -> HttpResponse {
  if let Err(error) = request.validate() {
    HttpResponse::Ok().json(error);
  }
  match conn(&app_data) {
    Ok(con) => {
      match db::insert_one(&request.enc_row(), &con) {
        Ok(data) => HttpResponse::Ok().json(data), 
        Err(err) => HttpResponse::Ok().json(err)
      }
    }
    Err(e) => HttpResponse::Ok().json(e)
  }
}