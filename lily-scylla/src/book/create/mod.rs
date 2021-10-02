pub mod create_and_update;

use actix_session::Session;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::App;
use validator::Validate;
use lily_utils::time_uuid;
use scylla::frame::response::cql_to_rust::FromRow;
use scylla::macros::FromRow;
use crate::utils::{
	GetQueryResult, 
	ConnectionResult
};
use crate::book::queries::{PARENT, CHILD};
use crate::auth::AuthSession;


#[derive(Deserialize, Validate, FromRow)]
pub struct ParentRequest {
    title: String,
    body: String,
    identity: i16,
}

#[derive(Serialize, Validate, FromRow)]
#[allow(non_snake_case)]
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

pub async fn parent_request(
    app: web::Data<App>, 
    request: web::Json<ParentRequest>,
    session: Session
) 
-> Result<HttpResponse, actix_web::Error> 
{
    
    let conn = app.conn_result()?;
    let auth = session.user_info()?;
    let auth_id_str = auth.userId;
    let auth_id = Uuid::parse_str(&auth_id_str).unwrap();
    let unique_id = time_uuid();
    let unique_id_str = unique_id.to_string();
    let auth_name = format!("{} {}", auth.fname, auth.lname);

    let _: Option<Vec<ParentRequest>> = conn
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


/*
    Child
*/

#[derive(Deserialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct ChildRequest {
    title: String,
    body: String,
    identity: i16,
    bookId: String,
    parentId: String,
}

#[derive(Serialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct ChildResponse {
    bookId: String,
    uniqueId: String,
    parentId: Option<String>,
    title: String,
    body: String,
    identity: i16,
    createdAt: String,
    updatedAt: String,
}

pub async fn child_request(
    app: web::Data<App>, 
    request: web::Json<ChildRequest>
) 
-> Result<HttpResponse, actix_web::Error> 
{
    let conn = app.conn_result()?;
    let unique_id = time_uuid();
    let unique_id_str = unique_id.to_string();
    let book_id = Uuid::parse_str(&request.bookId).unwrap();
    let parent_id = Uuid::parse_str(&request.parentId).unwrap();

    let _: Option<Vec<ChildRequest>> = conn
        .query(CHILD, 
            (
                &book_id, &unique_id, &parent_id,
                &request.title, &request.body,
                &request.identity, &unique_id, &unique_id,
            )
        ).await.get_query_result()?;

    Ok(
        HttpResponse::Ok().json(ChildResponse {
            bookId: unique_id_str.clone(),
            uniqueId: unique_id_str.clone(),
            parentId: None,
            title: request.title.clone(),
            body: request.body.clone(),
            identity: request.identity.clone(),
            createdAt: unique_id_str.clone(),
            updatedAt: unique_id_str.clone(),
        })
    )
}