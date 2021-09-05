use uuid::Uuid;
use serde::Deserialize;
use actix_web::{web, HttpResponse};
use serde::{Serialize};

// both of them is required to implement FromRow
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;

use crate::{App, utils::{ConnectionResult, GetQueryResult, Update}};

#[derive(Deserialize)]
pub struct UpdateDocumentData {
    bookId: Uuid,
    uniqueId: Uuid,
    title: String,
    body: String,
}

#[derive(FromRow, Serialize)]
pub struct Document {
	id: Uuid,
	title: String,
	tags: String,
    body: String,
}

pub async fn update_one(session: web::Data<App>, request: web::Json<UpdateDocumentData>) 
-> Result<HttpResponse, actix_web::Error> {
    let conn = session.conn_result()?;
    let query = Update::from("sankar.documents")
            .set("title", &request.title)
            .set("body", &request.body)
            .where_in("bookId", &request.bookId.to_string())
            .and("uniqueId", &request.uniqueId.to_string())
            .query();
    let _: Option<Vec<Document>> = conn
    .query(query, &[])
    .await.get_query_result()?;
    Ok(HttpResponse::Ok().body("Document updated"))
}
