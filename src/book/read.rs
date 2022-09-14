use actix_web::{HttpResponse,web};
use crate::App;
use uuid::Uuid;
use serde::Serialize;

use scylla::macros::FromRow;
use crate::utils::{
	GetQueryResult
};

#[derive(FromRow, Serialize)]
pub struct NewDocument {
    documentId: Uuid,
    title: String,
    tags: String,
    body: String
}

#[derive(FromRow, Serialize)]
pub struct Books {
    bookId: Uuid,
    authorId: Option<Uuid>,
    title: String,
    body: String,
    url: String,
    metadata: String,
    createdAt: Uuid,
    updatedAt: Uuid,
}

// cannot use * when getting all documents;
static BOOKS_QUERY: &'static str = "SELECT bookId, authorId, title, body, url, metadata, createdAt, updatedAt from sankar.books";
pub async fn getAllBooks(app: web::Data<App>) 
-> Result<HttpResponse, crate::AppError> {
    let documents: Option<Vec<Books>> = 
    app.query(BOOKS_QUERY, &[])
    .await
    .get_query_result()?;
    match documents {
        Some(docs) => Ok(HttpResponse::Ok().json(docs)),
        None => {
            let mt: Vec<Books> = Vec::new();
            Ok(HttpResponse::Ok().json(mt))
        },
    }
}

#[derive(FromRow, Serialize)]
pub struct Book {
    bookId: Uuid,
    uniqueId: Uuid,
    parentId: Option<Uuid>,
    authorId: Uuid,
    title: String,
    body: String,
    url: Option<String>,
    identity: i16,
    metadata: Option<String>,
    createdAt: Uuid,
    updatedAt: Uuid,
}

static GET_ALL_DOCUMENTS_FROM_ID: &'static str = "SELECT bookId, uniqueId, parentId, authorId, title, body, url, identity, metadata, createdAt, updatedAt from sankar.book WHERE bookId=";
pub async fn getAllNodesFromBookId(app: web::Data<App>, book_id: web::Path<String>) -> Result<HttpResponse, crate::AppError> {
    let bookId = Uuid::parse_str(&book_id)?;
    let query = format!("{}{}", GET_ALL_DOCUMENTS_FROM_ID, &bookId);
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