use actix_web::{web, HttpResponse};
use uuid::Uuid;
// use scylla::transport::session::{IntoTypedRows};
use crate::App;

pub async fn create_user(session: web::Data<App>) -> HttpResponse {
    let conn = session.session.get().unwrap();
    let id: Uuid = Uuid::new_v4();
    let fname: String = String::from("Arun");
    let lname: String = String::from("boro");
    let email: String = String::from("arun.boro@yahoo.com");
    let password: String = String::from("arun");

    conn
    .query("INSERT INTO sankar.users (id, email, fname, lname, password) VALUES(?,?,?,?,?)", (id, email, fname, lname, password))
    .await.unwrap();
    HttpResponse::Ok().body("New user created!")
}
