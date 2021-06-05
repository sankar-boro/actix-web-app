use actix_web::{HttpResponse,web};
use crate::App;
use uuid::Uuid;
use serde::Serialize;

use scylla::IntoTypedRows;
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;

#[derive(FromRow, Serialize)]
struct GetUser {
    id: Uuid,
    fname: Option<String>,
    lname: Option<String>,
    email: Option<String>,
}

pub async fn get_all(session: web::Data<App>) -> HttpResponse {
    let conn = session.session.get().unwrap();
    if let Some(rows) = conn.query("SELECT userId, fname, lname, email from sankar.users", &[]).await.unwrap().rows {
        let mut users = Vec::new();
        for row in rows.into_typed::<GetUser>() {
            let my_row: GetUser = row.unwrap();
            users.push(my_row);
        }
        return HttpResponse::Ok().json(users);
    }
    HttpResponse::Ok().body("Failed to get user")
}

pub async fn get_one(session: web::Data<App>, id: web::Path<String>,) -> HttpResponse {
    let conn = session.session.get().unwrap();
    let user_id =  Uuid::parse_str(&id).unwrap();
    if let Some(rows) = conn.query(format!("SELECT userId, fname, lname, email from sankar.users where userId={} LIMIT 1", user_id), &[]).await.unwrap().rows {
        let mut users = Vec::new();
        for row in rows.into_typed::<GetUser>() {
            let my_row: GetUser = row.unwrap();
            users.push(my_row);
        }
        return HttpResponse::Ok().json(users);
    }
    HttpResponse::Ok().body("Failed to get user")
}