use actix_web::{HttpResponse, web};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::App;
use validator::Validate;
use lily_utils::time_uuid;
use scylla::macros::FromRow;
use crate::query::{CREATE_NODE_QUERY};

#[derive(Deserialize, Validate, FromRow)]
pub struct UpdateRequest {
    title: String,
    body: String,
    bookId: String,
    uniqueId: String,
}

pub async fn update(
    app: web::Data<App>, 
    payload: web::Json<UpdateRequest>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let query = format!("UPDATE sankar.book SET title=?, body=? WHERE bookId=? AND uniqueId=?");
    app.query(query, (&payload.title, &payload.body, &payload.bookId, &payload.uniqueId)).await?;
    Ok(HttpResponse::Ok().body("Updated".to_string()))
}
