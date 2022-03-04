use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::App;

#[allow(dead_code)]
pub async fn delete_one(session: web::Data<App>, id: web::Path<String>) -> Result<HttpResponse, crate::AppError> {
    let user_id =  Uuid::parse_str(&id)?;
    
    session
    .query("DELETE FROM sankar.users WHERE id=?", (user_id,))
    .await?;
    Ok(HttpResponse::Ok().body("User deleted"))
}
