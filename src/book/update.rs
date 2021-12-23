use uuid::Uuid;
use serde::Deserialize;
use scylla::macros::FromRow;
use super::queries::UPDATE_BOOK;
use actix_web::{web, HttpResponse};


use crate::{App, utils::{ConnectionResult, GetQueryResult}};

#[derive(Deserialize, FromRow)]
#[allow(non_snake_case)]
pub struct Request {
    bookId: String,
    uniqueId: String,
    title: String,
    body: String,
}

pub async fn update_one(session: web::Data<App>, request: web::Json<Request>) 
-> Result<HttpResponse, actix_web::Error> {
    let conn = session.conn_result()?;
    let book_id = Uuid::parse_str(&request.bookId).unwrap();
    let unique_id = Uuid::parse_str(&request.uniqueId).unwrap();

    let _: Option<Vec<Request>> = conn
    .query(UPDATE_BOOK, (
        &request.title, &request.body, &book_id, &unique_id
    ))
    .await.get_query_result()?;
    
    Ok(HttpResponse::Ok().body("Document updated"))
}
