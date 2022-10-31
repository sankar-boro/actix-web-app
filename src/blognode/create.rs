use actix_web::{HttpResponse, web};
use serde::{Serialize, Deserialize};
use crate::App;
use validator::Validate;
use scylla::macros::FromRow;
use crate::query::{CREATE_BLOG_NODE_QUERY};
use crate::utils::ParseUuid;
use actix_session::Session;
use crate::{auth::AuthSession};
use uuid::Uuid;

#[derive(Deserialize, Validate, FromRow)]
pub struct AppendNodeRequest {
    uniqueId: String,
    title: String,
    body: String,
    blogId: String,
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

    let identity: i16 = 104;
    let new_id = &payload.uniqueId;
    let new_id = new_id.to_uuid()?;
    let blog_id = payload.blogId.to_uuid()?;
    let top_unique_id = payload.topUniqueId.to_uuid()?;
    let mut image_url = None;
    if let Some(b) = &payload.image_url {
        image_url = Some(b.to_owned());
    }
    let create_data = ( 
        &blog_id,
        &new_id,
        &top_unique_id,
        &author_id,
        &payload.title,
        &payload.body,
        &payload.metadata,
        &image_url,
        identity,
        &new_id,
        &new_id
    );
    app.query(CREATE_BLOG_NODE_QUERY, create_data).await?;
    Ok(HttpResponse::Ok().json(Response {
        uniqueId: payload.uniqueId.to_owned()
    }))
}
