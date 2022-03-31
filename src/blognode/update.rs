use actix_web::{HttpResponse, web};
use serde::Deserialize;
use crate::App;
use validator::Validate;
use scylla::macros::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Validate, FromRow)]
pub struct UpdateRequest {
    title: String,
    body: String,
    blogId: String,
    uniqueId: String,
}

pub async fn update(
    app: web::Data<App>, 
    payload: web::Json<UpdateRequest>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let blogId = Uuid::parse_str(&payload.blogId)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;
    let query = format!("UPDATE sankar.blog SET title=?, body=? WHERE blogId=? AND uniqueId=?");
    app.query(query, (&payload.title, &payload.body, &blogId, &uniqueId)).await?;
    Ok(HttpResponse::Ok().body("Updated".to_string()))
}
