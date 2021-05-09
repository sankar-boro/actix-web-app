use actix_web::{web, HttpResponse};
use scylla::{Session, SessionBuilder};
use std::{borrow::BorrowMut, sync::Arc};
use scylla::transport::session::{IntoTypedRows};
use crate::App;
use crate::user;

async fn home(session: web::Data<App>) -> HttpResponse {
  let conn = session.session.get().unwrap();
  if let Some(rows) = conn.query("SELECT id FROM sankar.users", &[]).await.unwrap().rows {
        // Parse each row as a tuple containing single i32
        for row in rows.into_typed::<(i32,)>() {
            let read_row: (i32,) = row.unwrap();
            println!("Read a value from row: {}", read_row.0);
        }
    }
  HttpResponse::Ok().body("Home!")
}

pub fn routes(config: &mut web::ServiceConfig) {
  config.route("/", web::get().to(home));
  config.service(web::scope("/user").route("/create", web::post().to(user::create_user)));
}

