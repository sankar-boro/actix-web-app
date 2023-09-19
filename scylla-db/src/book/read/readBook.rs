use actix_web::{HttpResponse,web};
use crate::Connections;
use uuid::Uuid;
use serde::{
    Serialize, 
    // Deserialize
};

use scylla::{macros::FromRow, query::Query};
use crate::utils::GetQueryResult;
use crate::query::{PAGE_SIZE, GET_SIZE};
use super::NextPageRequest;
use crate::Error;

#[derive(FromRow, Serialize)]
pub struct BookMetadata {
    docid: Uuid,
    authorId: i32,
    title: String,
    body: String,
    url: Option<String>,
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
static BOOKS_QUERY: &'static str = "SELECT docid, authorId, title, body, url, metadata, createdAt, updatedAt from sankar.books";
pub async fn getBooksWithPageSize(
    app: web::Data<Connections>
) 
-> Result<HttpResponse, Error> 
{
    let query = Query::new(BOOKS_QUERY).with_page_size(GET_SIZE);
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

// cannot use * when getting all documents;
pub async fn getNextBooksWithPageSize(
    app: web::Data<Connections>,
    request: web::Json<NextPageRequest>,
) 
-> Result<HttpResponse, Error> {
    let query = Query::new(BOOKS_QUERY).with_page_size(GET_SIZE);
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
    docid: Uuid,
    pageId: Uuid,
    uniqueId: Uuid,
    parentId: Option<Uuid>,
    authorId: i32,
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

static GET_BOOK_NODES_WITH_PAGE_SIZE: &'static str = "SELECT docid, pageId, uniqueId, parentId, authorId, title, body, url, identity, metadata, createdAt, updatedAt from sankar.book WHERE docid=? AND pageId=?";
pub async fn getBookNodesWithPageSizeFromId(
    app: web::Data<Connections>, 
    book_id: web::Path<String>
) -> Result<HttpResponse, Error> 
{
    let docid = Uuid::parse_str(&book_id)?;
    let documents= app.query(GET_BOOK_NODES_WITH_PAGE_SIZE, (&docid, &docid, ))
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

#[derive(Serialize)]
pub struct PageNodesResponse {
    nodes: Option<Vec<BookNode>>
}

static GET_PAGE_NODES_WITH_PAGE_SIZE: &'static str = "SELECT docid, pageId, uniqueId, parentId, authorId, title, body, url, identity, metadata, createdAt, updatedAt from sankar.book WHERE docid=? AND pageId=?";
pub async fn getBookNodesForPage(
    app: web::Data<Connections>, 
    ids: web::Path<(String, String)>
) -> Result<HttpResponse, Error> 
{
    let docid = Uuid::parse_str(&ids.0)?;
    let pageId = Uuid::parse_str(&ids.1)?;
    let nodes = app.query(GET_PAGE_NODES_WITH_PAGE_SIZE, (&docid, &pageId, ))
		.await?;
    let nodes: Option<Vec<BookNode>> = nodes.get_query_result()?;

    return Ok(HttpResponse::Ok().json(PageNodesResponse{ nodes }));
}

pub async fn getNextBookNodesWithPageSizeFromId(
    app: web::Data<Connections>, 
    book_id: web::Path<String>,
    request: web::Json<NextPageRequest>,
) -> Result<HttpResponse, Error> {
    let docid = Uuid::parse_str(&book_id)?;
    let query = format!("{}{}", GET_BOOK_NODES_WITH_PAGE_SIZE, &docid);
    let query = Query::new(query).with_page_size(PAGE_SIZE);
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