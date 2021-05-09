use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::App;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserData {
    fname: String,
}

pub async fn update_one(session: web::Data<App>, id: web::Path<String>, request: web::Form<UpdateUserData>) -> HttpResponse {
    let conn = session.session.get().unwrap();
    let user_id =  Uuid::parse_str(&id).unwrap();
    conn
    .query("UPDATE sankar.users SET fname=? WHERE id=?", (&request.fname, user_id,))
    .await.unwrap();
    HttpResponse::Ok().body("User updated")
}
