use actix_session::Session;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::{
    App,
    query::{ CREATE_BLOGS, CREATE_BLOG, CREATE_USER_BLOGS, CREATE_CATEGORY_BLOGS, CREATE_ALLCATEGORY }
};
use uuid::Uuid;
use scylla::{
    batch::Batch,
    macros::FromRow
};
use crate::auth::AuthSession;
use crate::utils::ParseUuid;

#[derive(Deserialize, FromRow)]
pub struct ParentRequest {
    title: String,
    body: Option<String>,
    metadata: String,
    uniqueId: String,
    category: String,
    image_url: Option<String>,
}

#[derive(Serialize)]
pub struct ParentResponse {
    blogId: String,
    uniqueId: String,
    parentId: Option<String>,
    authorId: String,
    title: String,
    body: String,
    url: Option<String>,
    identity: i16,
    metadata: String,
    createdAt: String,
    updatedAt: String,
}

pub async fn create(
    app: web::Data<App>, 
    request: web::Json<ParentRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    let mut batch: Batch = Default::default();
    batch.append_statement(CREATE_BLOGS);
    batch.append_statement(CREATE_BLOG);
    batch.append_statement(CREATE_USER_BLOGS);
    batch.append_statement(CREATE_CATEGORY_BLOGS);

    let identity: i16 = 101;
    let mut body = String::from("");
    let mut image_url = None;
    if let Some(b) = &request.body {
        body = b.to_owned();
    }
    if let Some(b) = &request.image_url {
        image_url = Some(b.to_owned());
    }

    let auth = session.user_info()?;
    let auth_id = &auth.userId.to_uuid()?;
    let unique_id = Uuid::parse_str(&request.uniqueId)?;
    let batch_values = (
        (&unique_id, &auth_id, &request.title, &body, &image_url, &request.metadata, &unique_id, &unique_id),
        (&unique_id, &unique_id, &auth_id, &request.title, &body, &image_url, &identity, &request.metadata, &unique_id, &unique_id),
        (&unique_id, &auth_id, &request.title, &body, &image_url, &request.metadata, &unique_id, &unique_id),
        (&request.category, &unique_id, &auth_id, &request.title, &body, &image_url, &request.metadata, &unique_id, &unique_id)
    );
    app.batch(&batch, &batch_values).await?;
    app.query(CREATE_ALLCATEGORY, (&request.category, "demo")).await?;
    Ok(
        HttpResponse::Ok().json(ParentResponse {
            blogId: request.uniqueId.clone(),
            uniqueId: request.uniqueId.clone(),
            parentId: None,
            title: request.title.clone(),
            body: body.clone(),
            url: image_url.clone(),
            identity,
            authorId: auth_id.to_string(),
            metadata: request.metadata.clone(),
            createdAt: request.uniqueId.clone(),
            updatedAt: request.uniqueId.clone(),
        })
    )
}
