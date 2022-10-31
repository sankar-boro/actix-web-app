use actix_web::{HttpResponse, web};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{App, auth::AuthSession};
use crate::utils::ParseUuid;
use scylla::macros::FromRow;
use crate::query::{CREATE_BOOK_NODE_QUERY};
use actix_session::Session;

#[derive(Deserialize, FromRow)]
pub struct AppendNodeRequest {
    uniqueId: String,
    title: String,
    body: String,
    identity: i16,
    bookId: String,
    topUniqueId: String,
    metadata: String,
    image_url: Option<String>,
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
    let new_id = &payload.uniqueId;
    let new_id = new_id.to_uuid()?;
    let book_id = &payload.bookId.to_uuid()?;
    let top_unique_id = &payload.topUniqueId.to_uuid()?;
    let mut image_url = None;
    if let Some(b) = &payload.image_url {
        image_url = Some(b.to_owned());
    }
    let create_data = ( 
        &book_id,
        &new_id,
        &top_unique_id,
        &author_id,
        &payload.title,
        &payload.body,
        &payload.metadata,
        &image_url,
        &payload.identity,
        &new_id,
        &new_id
    );
    app.query(CREATE_BOOK_NODE_QUERY, create_data).await?;
    Ok(HttpResponse::Ok().json(Response {
        uniqueId: payload.uniqueId.to_owned()
    }))
}
