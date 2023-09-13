use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::Connections;
use crate::Error;

#[allow(dead_code)]
pub async fn delete_one(session: web::Data<Connections>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let user_id =  Uuid::parse_str(&id)?;
    
    session
    .query("DELETE FROM sankar.users WHERE id=?", (user_id,))
    .await?;
    Ok(HttpResponse::Ok().body("User deleted"))
}
