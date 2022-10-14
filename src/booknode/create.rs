use actix_web::{HttpResponse, web};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{App, auth::AuthSession};
use crate::utils::ParseUuid;
use lily_utils::time_uuid;
use scylla::macros::FromRow;
use crate::query::{CREATE_BOOK_NODE_QUERY};
use actix_session::Session;

#[derive(Deserialize, FromRow)]
pub struct AppendNodeRequest {
    title: String,
    body: String,
    identity: i16,
    bookId: String,
    topUniqueId: String,
    metadata: String,
}

#[derive(Serialize)]
pub struct Response {
    uniqueId: String,
}

pub async fn create(
    app: web::Data<App>, 
    payload: web::Json<AppendNodeRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let auth = session.user_info()?;
    let author_id = Uuid::parse_str(&auth.userId)?;
    let new_id = time_uuid();
    let book_id = &payload.bookId.to_uuid()?;
    let top_unique_id = &payload.topUniqueId.to_uuid()?;
    let unique_id = new_id.to_string();
    let create_data = ( 
        &book_id,
        &new_id,
        &top_unique_id,
        &author_id,
        &payload.title,
        &payload.body,
        &payload.metadata,
        &payload.identity,
        &new_id,
        &new_id
    );
    app.query(CREATE_BOOK_NODE_QUERY, create_data).await?;
    Ok(HttpResponse::Ok().json(Response {
        uniqueId: unique_id
    }))
}
