use actix_session::Session;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::App;
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
    image_url: Option<String>,
}

#[derive(Serialize)]
pub struct ParentResponse {
    blogId: String,
    uniqueId: String,
    parentId: Option<String>,
    title: String,
    body: String,
    authorId: String,
    identity: i16,
    fname: String,
    lname: String,
    createdAt: String,
    updatedAt: String,
}

pub static CREATE_BLOGS: &str = "INSERT INTO sankar.blogs (
    blogId, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";
pub static CREATE_BLOG: &str = "INSERT INTO sankar.blog (
    blogId, uniqueId, authorId, title, body, url, identity, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)";
pub static CREATE_USER_BLOGS: &str = "INSERT INTO sankar.userblogs (
    blogId, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";

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
    let identity: i16 = 101;

    let mut body = String::from("");
    let mut image_url = String::from("");

    if let Some(b) = &request.body {
        body = b.to_owned();
    }
    if let Some(b) = &request.image_url {
        image_url = b.to_owned();
    }

    let auth = session.user_info()?;
    let auth_id = &auth.userId.to_uuid()?;
    let unique_id = Uuid::parse_str(&request.uniqueId)?;

    let batch_values = (
        (&unique_id, &auth_id, &request.title, &body, &image_url, &request.metadata, &unique_id, &unique_id),
        (&unique_id, &unique_id, &auth_id, &request.title, &body, &image_url, &identity, &request.metadata, &unique_id, &unique_id),
        (&unique_id, &auth_id, &request.title, &body, &image_url, &request.metadata, &unique_id, &unique_id)
    );

    app.batch(&batch, &batch_values).await?;

    Ok(
        HttpResponse::Ok().json(ParentResponse {
            blogId: request.uniqueId.clone(),
            uniqueId: request.uniqueId.clone(),
            parentId: None,
            title: request.title.clone(),
            body: body.clone(),
            identity,
            authorId: auth_id.to_string(),
            fname: auth.fname.clone(),
            lname: auth.lname.clone(),
            createdAt: request.uniqueId.clone(),
            updatedAt: request.uniqueId.clone(),
        })
    )
}
