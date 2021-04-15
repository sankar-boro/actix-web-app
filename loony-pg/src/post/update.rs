use actix_web::{HttpResponse, web};
use validator::Validate;
use super::schema::posts;
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

pub async fn update_one(post_id: web::Path<i32>, request: web::Form<UpdatePost>, app_data: web::Data<App>) -> HttpResponse {
  if let Err(error) = request.validate() {
    HttpResponse::Ok().json(error);
  }

  match conn(&app_data) {
    Ok(con) => {
      match db::update_one(post_id.0, &request, &con) {
        Ok(_) => HttpResponse::Ok().body("Updated"), 
        Err(err) => HttpResponse::Ok().json(err)
      }
    }
    Err(e) => HttpResponse::Ok().json(e)
  }

}