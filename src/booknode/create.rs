use actix_web::{HttpResponse, web};
use serde::{Serialize, Deserialize};
use crate::App;
use crate::utils::ParseUuid;
use lily_utils::time_uuid;
use scylla::macros::FromRow;
use crate::query::{CREATE_BOOK_NODE_QUERY};

#[derive(Deserialize, FromRow)]
pub struct AppendNodeRequest {
    title: String,
    body: String,
    identity: i16,
    bookId: String,
    topUniqueId: String,
}

#[derive(Serialize)]
pub struct Response {
    uniqueId: String,
}

pub async fn create(
    app: web::Data<App>, 
    payload: web::Json<AppendNodeRequest>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let new_id = time_uuid();
    let book_id = &payload.bookId.to_uuid()?;
    let top_unique_id = &payload.topUniqueId.to_uuid()?;
    let unique_id = new_id.to_string();
    let create_data = ( 
        &book_id,
        &new_id,
        &top_unique_id,
        &payload.title,
        &payload.body,
        &payload.identity,
        &new_id,
        &new_id
    );
    app.query(CREATE_BOOK_NODE_QUERY, create_data).await?;
    Ok(HttpResponse::Ok().json(Response {
        uniqueId: unique_id
    }))
}
