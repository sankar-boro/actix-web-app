use actix_web::{HttpResponse, web};
use lily_utils::time_uuid;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{App, auth::AuthSession};
use crate::utils::ParseUuid;
use scylla::macros::FromRow;
use actix_session::Session;
use crate::create_query;

#[derive(Deserialize, FromRow)]
pub struct AppendNodeRequest {
    body: String,
    identity: i16,
    bookId: String,
    parentId: String
}

#[derive(Serialize)]
pub struct Response {
    uniqueId: String,
}

lazy_static! {
    static ref QUERY: String = create_query!("sankar.updatebooknode", "bookId", "uniqueId", "parentId", "authorId", "body", "identity", "createdAt", "updatedAt");
}
pub async fn pull_request(
    app: web::Data<App>, 
    payload: web::Json<AppendNodeRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let new_id = time_uuid();
    let auth = session.user_info()?;
    let author_id = Uuid::parse_str(&auth.userId)?;
    let book_id = &payload.bookId.to_uuid()?;
    let parent_id = &payload.parentId.to_uuid()?;

    let create_data = ( 
        &book_id,
        &new_id,
        &parent_id,
        &author_id,
        &payload.body,
        &payload.identity,
        &new_id,
        &new_id
    );
    app.query(QUERY.as_str(), create_data).await?;
    Ok(HttpResponse::Ok().json(Response {
        uniqueId: new_id.to_string()
    }))
}
