use actix_web::{HttpResponse, web};
use serde::{Serialize, Deserialize};
use crate::App;
use validator::Validate;
use lily_utils::time_uuid;
use scylla::macros::FromRow;
use crate::query::{CREATE_BLOG_NODE_QUERY};
use crate::utils::ParseUuid;

#[derive(Deserialize, Validate, FromRow)]
pub struct AppendNodeRequest {
    title: String,
    body: String,
    blogId: String,
    topUniqueId: String,
    metadata: String,
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
    let identity: i16 = 104;
    let new_id = time_uuid();
    let blog_id = payload.blogId.to_uuid()?;
    let top_unique_id = payload.topUniqueId.to_uuid()?;
    let unique_id = new_id.to_string();
    let create_data = ( 
        &blog_id,
        &new_id,
        &top_unique_id,
        &payload.title,
        &payload.body,
        &payload.metadata,
        identity,
        &new_id,
        &new_id
    );
    app.query(CREATE_BLOG_NODE_QUERY, create_data).await?;
    Ok(HttpResponse::Ok().json(Response {
        uniqueId: unique_id
    }))
}
