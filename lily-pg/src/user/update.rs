use actix_web::{HttpResponse, web};
use validator::Validate;
use super::schema::users;
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

pub async fn update_one(user_id: web::Path<i32>, request: web::Form<UpdateUser>, app_data: web::Data<App>) -> HttpResponse {
  if let Err(error) = request.validate() {
    HttpResponse::Ok().json(error);
  }

  match conn(&app_data) {
    Ok(con) => {
      match db::update_one(user_id.0, &request, &con) {
        Ok(_) => HttpResponse::Ok().body("Updated"), 
        Err(err) => HttpResponse::Ok().json(err)
      }
    }
    Err(e) => HttpResponse::Ok().json(e)
  }
}