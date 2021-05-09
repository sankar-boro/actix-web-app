use actix_web::{web, HttpResponse};
use serde::{Deserialize};
use uuid::Uuid;
// use scylla::transport::session::{IntoTypedRows};
use crate::App;
use validator::Validate;
use loony_service::encrypt_text;

#[derive(Deserialize, Validate)]
pub struct SignupFormData {
    fname: String,
    lname: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

pub async fn create_user(session: web::Data<App>, request: web::Form<SignupFormData>) -> HttpResponse {
    let conn = session.session.get().unwrap();
    let id: Uuid = Uuid::new_v4();
    let password = encrypt_text(&request.password);
    conn
    .query("INSERT INTO sankar.users (id, email, fname, lname, password) VALUES(?,?,?,?,?)", (id, &request.email, &request.fname, &request.lname, password))
    .await.unwrap();
    HttpResponse::Ok().body("New user created!")
}
