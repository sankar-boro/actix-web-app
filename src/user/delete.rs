use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::App;

#[allow(dead_code)]
pub async fn delete_one(session: web::Data<App>, id: web::Path<String>) -> HttpResponse {
    let user_id =  Uuid::parse_str(&id).unwrap();
    session
    .session
    .query("DELETE FROM sankar.users WHERE id=?", (user_id,))
    .await.unwrap();
    HttpResponse::Ok().body("User deleted")
}
