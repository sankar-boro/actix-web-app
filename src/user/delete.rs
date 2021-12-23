use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::App;

#[allow(dead_code)]
pub async fn delete_one(session: web::Data<App>, id: web::Path<String>) -> HttpResponse {
    let conn = session.session.get().unwrap();
    let user_id =  Uuid::parse_str(&id).unwrap();
    conn
    .query("DELETE FROM sankar.users WHERE id=?", (user_id,))
    .await.unwrap();
    HttpResponse::Ok().body("User deleted")
}
