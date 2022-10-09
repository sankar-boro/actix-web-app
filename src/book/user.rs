use actix_web::{HttpResponse,web};
use crate::App;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use scylla::{macros::FromRow, query::Query};
use crate::utils::{
	GetQueryResult
};
use crate::query::{GET_SIZE};


#[derive(FromRow, Serialize)]
pub struct BookMetadata {
    bookId: Uuid,
    authorId: Option<Uuid>,
    title: String,
    body: String,
    url: Option<String>,
    metadata: String,
    createdAt: Uuid,
    updatedAt: Uuid,
}

#[derive(Serialize)]
pub struct BooksResponse {
    books: Vec<BookMetadata>,
    page: Option<Vec<u8>>
}

#[derive(FromRow, Serialize)]
pub struct BlogMetadata {
    blogId: Uuid,
    authorId: Option<Uuid>,
    title: String,
    body: String,
    url: Option<String>,
    metadata: String,
    createdAt: Uuid,
    updatedAt: Uuid,
}
#[derive(Serialize)]
pub struct BlogsResponse {
    blogs: Vec<BlogMetadata>,
    page: Option<Vec<u8>>
}

// cannot use * when getting all documents;
static GET_ALL_BOOKS_FROM_ID: &'static str = "SELECT bookId, authorId, title, body, url, metadata, createdAt, updatedAt from sankar.userbooks WHERE authorId=";
pub async fn getPagedBooksForAuthorId(app: web::Data<App>, author_id: web::Path<String>) -> Result<HttpResponse, crate::AppError> {
    let authorId = Uuid::parse_str(&author_id)?;
    let query = format!("{}{}", GET_ALL_BOOKS_FROM_ID, &authorId);
    let query = Query::new(query).with_page_size(GET_SIZE);

    let documents = 
		app.query(query, &[])
		.await?;
    let page = documents.paging_state.clone();
    let documents: Option<Vec<BookMetadata>> = documents.get_query_result()?; 

    match documents {
        Some(books) => {
            let page = match page {
                Some(page) => Some(page.to_vec()),
                None => None,
            };
            Ok(HttpResponse::Ok().json(BooksResponse { books, page }))
        },
        None => {
            let x: Vec<BookMetadata> = Vec::new();
            let y = BooksResponse{books: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}

// cannot use * when getting all documents;
static GET_ALL_BLOGS_FROM_ID: &'static str = "SELECT blogId, authorId, title, body, url, metadata, createdAt, updatedAt from sankar.userblogs WHERE authorId=";
pub async fn getPagedBlogsForAuthorId(app: web::Data<App>, author_id: web::Path<String>) -> Result<HttpResponse, crate::AppError> {
    let authorId = Uuid::parse_str(&author_id)?;
    let query = format!("{}{}", GET_ALL_BLOGS_FROM_ID, &authorId);
    let query = Query::new(query).with_page_size(GET_SIZE);
    let documents = app.query(query, &[])
		.await?;
    let page = documents.paging_state.clone();
	let documents: Option<Vec<BlogMetadata>> = documents.get_query_result()?;

    match documents {
        Some(blogs) => {
            let page = match page {
                Some(page) => Some(page.to_vec()),
                None => None,
            };
            Ok(HttpResponse::Ok().json(BlogsResponse { blogs, page }))
        },
        None => {
            let x: Vec<BlogMetadata> = Vec::new();
            let y = BlogsResponse{blogs: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}

#[derive(Serialize, Deserialize)]
pub struct NextPageRequest {
    page: Vec<u8>
}

pub async fn getNextPageBooksForAuthorId(
    app: web::Data<App>, 
    author_id: web::Path<String>,
    request: web::Json<NextPageRequest>,
) -> Result<HttpResponse, crate::AppError> 
{
    let authorId = Uuid::parse_str(&author_id)?;
    let query = format!("{}{}", GET_ALL_BOOKS_FROM_ID, &authorId);
    let query = Query::new(query).with_page_size(GET_SIZE);

    let documents = 
		app.query_paged(query, &[], request.page.clone())
		.await?;
    let page = documents.paging_state.clone();
    let documents: Option<Vec<BookMetadata>> = documents.get_query_result()?; 

    match documents {
        Some(books) => {
            let page = match page {
                Some(page) => Some(page.to_vec()),
                None => None,
            };
            Ok(HttpResponse::Ok().json(BooksResponse { books, page }))
        },
        None => {
            let x: Vec<BookMetadata> = Vec::new();
            let y = BooksResponse{books: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}

pub async fn getNextPageBlogsForAuthorId(
    app: web::Data<App>, 
    author_id: web::Path<String>,
    request: web::Json<NextPageRequest>,
) -> Result<HttpResponse, crate::AppError> 
{
    let authorId = Uuid::parse_str(&author_id)?;
    let query = format!("{}{}", GET_ALL_BLOGS_FROM_ID, &authorId);
    let query = Query::new(query).with_page_size(GET_SIZE);
    let documents = app.query_paged(query, &[], request.page.clone())
		.await?;
    let page = documents.paging_state.clone();
	let documents: Option<Vec<BlogMetadata>> = documents.get_query_result()?;

    match documents {
        Some(blogs) => {
            let page = match page {
                Some(page) => Some(page.to_vec()),
                None => None,
            };
            Ok(HttpResponse::Ok().json(BlogsResponse { blogs, page }))
        },
        None => {
            let x: Vec<BlogMetadata> = Vec::new();
            let y = BlogsResponse{blogs: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}