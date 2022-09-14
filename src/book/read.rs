use actix_web::{HttpResponse,web};
use crate::App;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use scylla::{macros::FromRow, query::Query, Bytes};
use crate::utils::{
	GetQueryResult
};

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

#[derive(Serialize)]
pub struct Res {
    page: Option<Vec<u8>>,
    data: Vec<Book>
}

static GET_ALL_DOCUMENTS_FROM_ID: &'static str = "SELECT bookId, uniqueId, parentId, authorId, title, body, url, identity, metadata, createdAt, updatedAt from sankar.book WHERE bookId=";
pub async fn getAllNodesFromBookId(app: web::Data<App>, book_id: web::Path<String>) -> Result<HttpResponse, crate::AppError> {
    let bookId = Uuid::parse_str(&book_id)?;
    let query = format!("{}{}", GET_ALL_DOCUMENTS_FROM_ID, &bookId);
    let query = Query::new(query).with_page_size(3);
    let documents= app.query(query, &[])
		.await?;
	let pagination = documents.paging_state.clone();
    let documents: Option<Vec<Book>> = documents.get_query_result()?;

    match documents {
        Some(data) => {
            let page = match pagination {
                Some(data) => {
                    Some(data.to_vec())
                },
                None => None,
            };
            return Ok(HttpResponse::Ok().json(Res{page, data }));
        },
        None => {
            let mt: Vec<Book> = Vec::new();
            Ok(HttpResponse::Ok().json(mt))
        },
    }
}

#[derive(Serialize, Deserialize)]
pub struct NextPageBook {
    page: Vec<u8>
}

static NEXT_PAGE_BOOK: &'static str = "SELECT bookId, uniqueId, parentId, authorId, title, body, url, identity, metadata, createdAt, updatedAt from sankar.book WHERE bookId=";
pub async fn getNextPage(
    app: web::Data<App>, 
    book_id: web::Path<String>,
    request: web::Json<NextPageBook>,
) -> Result<HttpResponse, crate::AppError> {
    let bookId = Uuid::parse_str(&book_id)?;
    let query = format!("{}{}", NEXT_PAGE_BOOK, &bookId);
    let query = Query::new(query).with_page_size(3);
    let page: Vec<u8> = request.page.clone();
    let documents= app.query_paged(query, &[], page)
		.await?;
	let pagination = documents.paging_state.clone();
    let documents: Option<Vec<Book>> = documents.get_query_result()?;

    match documents {
        Some(data) => {
            let page = match pagination {
                Some(data) => {
                    Some(data.to_vec())
                },
                None => None,
            };
            return Ok(HttpResponse::Ok().json(Res{page, data }));
        },
        None => {
            let mt: Vec<Book> = Vec::new();
            Ok(HttpResponse::Ok().json(mt))
        },
    }
}