use serde::{Deserialize};
use validator::Validate;
use actix_web::{web, HttpResponse};
use crate::connection::conn;
use crate::App;

use super::{db, schema::posts};

trait Row {
  fn into_row(&self) -> &Self;
}

#[derive(Deserialize, Debug, Insertable, Validate, Clone)]
#[table_name = "posts"]
pub struct CreatePost {
  user_id: i32,
  title: String,
  body: String,
  image: Option<String>
}

impl Row for CreatePost {
  fn into_row(&self) -> &Self {
    self
  }
}

pub fn create(request: web::Form<CreatePost>, app_data: web::Data<App>) -> HttpResponse {
  match conn(&app_data) {
    Ok(con) =>  {
      match db::insert(request.into_row(), &con) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(n) => HttpResponse::Ok().json(n)
      }
    }
    Err(e) => HttpResponse::Ok().json(e)
  }
}
