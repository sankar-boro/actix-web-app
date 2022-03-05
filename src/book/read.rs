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
pub struct BookInfo {
    bookId: Uuid,
    authorId: Option<Uuid>,
    fname: Option<String>,
    lname: Option<String>,
    title: String,
    body: String,
    createdAt: Uuid,
    updatedAt: Uuid,
}

// cannot use * when getting all documents;
static GET_ALL_DOCUMENTS: &'static str = "SELECT bookId, authorId, fname, lname, title, body, createdAt, updatedAt from sankar.bookInfo";
pub async fn getAllBooks(app: web::Data<App>) 
-> Result<HttpResponse, crate::AppError> {
    let documents: Option<Vec<BookInfo>> = 
    app.query(GET_ALL_DOCUMENTS, &[])
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

#[derive(FromRow, Serialize)]
pub struct Book {
    bookId: Uuid,
    uniqueId: Uuid,
    parentId: Option<Uuid>,
    authorId: Option<Uuid>,
    fname: Option<String>,
    lname: Option<String>,
    title: String,
    body: String,
    identity: i16,
    createdAt: Uuid,
    updatedAt: Uuid,
}

static GET_ALL_DOCUMENTS_FROM_ID: &'static str = "SELECT bookId, uniqueId, parentId, authorId, fname, lname, title, body, identity, createdAt, updatedAt from sankar.book WHERE bookId=";
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