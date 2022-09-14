use actix_web::{HttpResponse,web};
use crate::App;
use uuid::Uuid;
use serde::Serialize;

use scylla::macros::FromRow;
use crate::utils::{
	GetQueryResult
};


#[derive(FromRow, Serialize)]
pub struct Book {
    bookId: Uuid,
    authorId: Option<Uuid>,
    title: String,
    body: String,
    url: Option<String>,
    metadata: String,
    createdAt: Uuid,
    updatedAt: Uuid,
}


// cannot use * when getting all documents;
static GET_ALL_DOCUMENTS_FROM_ID: &'static str = "SELECT bookId, authorId, title, body, url, metadata, createdAt, updatedAt from sankar.userbooks WHERE authorId=";
pub async fn getAllNodesFromAuthorId(app: web::Data<App>, author_id: web::Path<String>) -> Result<HttpResponse, crate::AppError> {
    let authorId = Uuid::parse_str(&author_id)?;
    let query = format!("{}{}", GET_ALL_DOCUMENTS_FROM_ID, &authorId);
    let documents: Option<Vec<Book>> = 
		app.query(query, &[])
		.await
		.get_query_result()?;

    match documents {
        Some(docs) => Ok(HttpResponse::Ok().json(docs)),
        None => {
            let mt: Vec<Book> = Vec::new();
            Ok(HttpResponse::Ok().json(mt))
        },
    }
}