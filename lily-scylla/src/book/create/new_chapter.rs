use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::App;
use validator::Validate;
use lily_utils::time_uuid;
use scylla::frame::response::cql_to_rust::FromRow;
use scylla::macros::FromRow;
use crate::utils::{
	GetQueryResult, 
	ConnectionResult
};
use crate::book::queries::CHILD;


#[derive(Deserialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct Request {
    title: String,
    body: String,
    identity: i16,
    bookId: String,
    parentId: String,
}

#[derive(Serialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct Response {
    bookId: String,
    uniqueId: String,
    parentId: Option<String>,
    title: String,
    body: String,
    identity: i16,
    createdAt: String,
    updatedAt: String,
}

pub async fn create_new_chapter(
    app: web::Data<App>, 
    request: web::Json<Request>
) 
-> Result<HttpResponse, actix_web::Error> 
{
    let conn = app.conn_result()?;
    let unique_id = time_uuid();
    let unique_id_str = unique_id.to_string();

    let _: Option<Vec<Request>> = conn
        .query(CHILD, 
            (
                &request.bookId, &unique_id, &request.parentId,
                &request.title, &request.body,
                &request.identity, &unique_id, &unique_id,
            )
        ).await.get_query_result()?;

    Ok(
        HttpResponse::Ok().json(Response {
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