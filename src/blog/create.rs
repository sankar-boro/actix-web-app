use actix_session::Session;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::App;
use lily_utils::time_uuid;
use scylla::{
    batch::Batch,
    macros::FromRow
};
use crate::auth::AuthSession;
use crate::utils::ParseUuid;

#[derive(Deserialize, FromRow)]
pub struct ParentRequest {
    title: String,
    body: String,
}

#[derive(Serialize)]
pub struct ParentResponse {
    blogId: String,
    uniqueId: String,
    parentId: Option<String>,
    title: String,
    body: String,
    authorId: String,
    identity: String,
    fname: String,
    lname: String,
    createdAt: String,
    updatedAt: String,
}

pub static CREATE_BLOG: &str = "INSERT INTO sankar.blog (
    blogId, uniqueId, authorId, fname, lname, title, body, identity, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_BLOG_INFO: &str = "INSERT INTO sankar.blogInfo (
    blogId, authorId, fname, lname, title, body, createdAt, updatedAt
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
    batch.append_statement(CREATE_BLOG);
    batch.append_statement(CREATE_BLOG_INFO);
    let identity = "101".to_string();

    let auth = session.user_info()?;
    let auth_id = &auth.userId.to_uuid()?;
    let unique_id = time_uuid();
    let unique_id_str = unique_id.to_string();

    let batch_values = (
        (&unique_id, &unique_id, &auth_id, &auth.fname, &auth.lname, &request.title, &request.body, &identity, &unique_id, &unique_id),
        (&unique_id, &auth_id, &auth.fname, &auth.lname, &request.title, &request.body, &unique_id, &unique_id)
    );

    app.batch(&batch, &batch_values).await?;

    Ok(
        HttpResponse::Ok().json(ParentResponse {
            blogId: unique_id_str.clone(),
            uniqueId: unique_id_str.clone(),
            parentId: None,
            title: request.title.clone(),
            body: request.body.clone(),
            identity,
            authorId: auth_id.to_string(),
            fname: auth.fname.clone(),
            lname: auth.lname.clone(),
            createdAt: unique_id_str.clone(),
            updatedAt: unique_id_str.clone(),
        })
    )
}