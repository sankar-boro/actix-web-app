use actix_session::Session;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::App;
use validator::Validate;
use lily_utils::time_uuid;
use scylla::macros::FromRow;
use crate::utils::{
	GetQueryResult
};
use crate::book::queries::PARENT;
use crate::auth::AuthSession;

#[derive(Deserialize, Validate, FromRow)]
pub struct ParentRequest {
    title: String,
    body: String,
    identity: i16,
}

#[derive(Serialize, Validate, FromRow)]
pub struct ParentResponse {
    bookId: String,
    uniqueId: String,
    parentId: Option<String>,
    title: String,
    body: String,
    identity: i16,
    authorId: String,
    authorName: String,
    createdAt: String,
    updatedAt: String,
}

pub async fn new_book(
    app: web::Data<App>, 
    request: web::Json<ParentRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    let auth = session.user_info()?;
    let auth_id_str = auth.userId;
    let auth_id = Uuid::parse_str(&auth_id_str).unwrap();
    let unique_id = time_uuid();
    let unique_id_str = unique_id.to_string();
    let auth_name = format!("{} {}", auth.fname, auth.lname);
    let _: Option<Vec<ParentRequest>> = app.session
        .query(PARENT, 
            (
                &unique_id, &unique_id, &auth_id,
                &auth_name, &request.title, &request.body,
                &request.identity, &unique_id, &unique_id,
            )
        ).await.get_query_result()?;

    Ok(
        HttpResponse::Ok().json(ParentResponse {
            bookId: unique_id_str.clone(),
            uniqueId: unique_id_str.clone(),
            parentId: None,
            title: request.title.clone(),
            body: request.body.clone(),
            identity: request.identity.clone(),
            authorId: auth_id.to_string(),
            authorName: auth_name,
            createdAt: unique_id_str.clone(),
            updatedAt: unique_id_str.clone(),
        })
    )
}