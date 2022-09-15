use actix_web::{HttpResponse,web};
use crate::App;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use scylla::{macros::FromRow, query::Query};
use crate::utils::{
	GetQueryResult
};

#[derive(FromRow, Serialize)]
pub struct BookMetadata {
    bookId: Uuid,
    authorId: Uuid,
    title: String,
    body: String,
    url: String,
    metadata: String,
    createdAt: Uuid,
    updatedAt: Uuid,
}

#[derive(Serialize)]
pub struct BooksMetadataResponse {
    books: Vec<BookMetadata>,
    page: Option<Vec<u8>>,
}

// cannot use * when getting all documents;
static BOOKS_QUERY: &'static str = "SELECT bookId, authorId, title, body, url, metadata, createdAt, updatedAt from sankar.books";
pub async fn getBooksWithPageSize(
    app: web::Data<App>
) 
-> Result<HttpResponse, crate::AppError> 
{
    let query = Query::new(BOOKS_QUERY).with_page_size(4);
    let documents = app.query(query, &[])
    .await?;
    let page = documents.paging_state.clone();
    let documents: Option<Vec<BookMetadata>> = documents.get_query_result()?;
    match documents {
        Some(docs) => {
            let page = match page {
                Some(page) => Some(page.to_vec()),
                None => None,
            };
            Ok(HttpResponse::Ok().json(BooksMetadataResponse{books: docs, page }))
        },
        None => {
            let x: Vec<BookMetadata> = Vec::new();
            let y = BooksMetadataResponse{books: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}


#[derive(Serialize, Deserialize)]
pub struct NextPageRequest {
    page: Vec<u8>
}

// cannot use * when getting all documents;
pub async fn getNextBooksWithPageSize(
    app: web::Data<App>,
    request: web::Json<NextPageRequest>,
) 
-> Result<HttpResponse, crate::AppError> {
    let query = Query::new(BOOKS_QUERY).with_page_size(4);
    let documents = app.query_paged(query, &[], request.page.clone())
    .await?;
    let page = documents.paging_state.clone();
    let documents: Option<Vec<BookMetadata>> = documents.get_query_result()?;
    match documents {
        Some(docs) => {
            let page = match page {
                Some(page) => Some(page.to_vec()),
                None => None,
            };
            Ok(HttpResponse::Ok().json(BooksMetadataResponse{books: docs, page }))
        },
        None => {
            let x: Vec<BookMetadata> = Vec::new();
            let y = BooksMetadataResponse{books: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}

#[derive(FromRow, Serialize)]
pub struct BookNode {
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
pub struct BookNodesResponse {
    nodes: Vec<BookNode>,
    page: Option<Vec<u8>>
}

static GET_BOOK_NODES_WITH_PAGE_SIZE: &'static str = "SELECT bookId, uniqueId, parentId, authorId, title, body, url, identity, metadata, createdAt, updatedAt from sankar.book WHERE bookId=";
pub async fn getBookNodesWithPageSizeFromId(
    app: web::Data<App>, 
    book_id: web::Path<String>
) -> Result<HttpResponse, crate::AppError> 
{
    let bookId = Uuid::parse_str(&book_id)?;
    let query = format!("{}{}", GET_BOOK_NODES_WITH_PAGE_SIZE, &bookId);
    let query = Query::new(query).with_page_size(3);
    let documents= app.query(query, &[])
		.await?;
	let page = documents.paging_state.clone();
    let documents: Option<Vec<BookNode>> = documents.get_query_result()?;

    match documents {
        Some(nodes) => {
            let page = match page {
                Some(data) => {
                    Some(data.to_vec())
                },
                None => None,
            };
            return Ok(HttpResponse::Ok().json(BookNodesResponse{nodes, page }));
        },
        None => {
            let x: Vec<BookNode> = Vec::new();
            let y = BookNodesResponse{nodes: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}

pub async fn getNextBookNodesWithPageSizeFromId(
    app: web::Data<App>, 
    book_id: web::Path<String>,
    request: web::Json<NextPageRequest>,
) -> Result<HttpResponse, crate::AppError> {
    let bookId = Uuid::parse_str(&book_id)?;
    let query = format!("{}{}", GET_BOOK_NODES_WITH_PAGE_SIZE, &bookId);
    let query = Query::new(query).with_page_size(3);
    let page: Vec<u8> = request.page.clone();
    let documents= app.query_paged(query, &[], page)
		.await?;
	let page = documents.paging_state.clone();
    let documents: Option<Vec<BookNode>> = documents.get_query_result()?;

    match documents {
        Some(nodes) => {
            let page = match page {
                Some(page) => {
                    Some(page.to_vec())
                },
                None => None,
            };
            return Ok(HttpResponse::Ok().json(BookNodesResponse{nodes,page}));
        },
        None => {
            let x: Vec<BookNode> = Vec::new();
            let y = BookNodesResponse{nodes: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}