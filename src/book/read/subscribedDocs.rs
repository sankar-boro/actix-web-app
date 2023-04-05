use actix_web::{HttpResponse,web};
use crate::Connections;
use uuid::Uuid;
use serde::{
    Serialize, 
    // Deserialize
};

use scylla::{macros::FromRow, query::Query};
use crate::utils::{
	GetQueryResult
};
use crate::query::{
    // PAGE_SIZE, 
    GET_SIZE
};
use super::NextPageRequest;

// cannot use * when getting all documents;
static BOOKS_QUERY_CATEGORY: &'static str = "SELECT category, bookId, authorId, title, body, url, metadata, createdAt, updatedAt from sankar.categorybooks WHERE category";
pub async fn getBooksWithPageSizeCategories(
    app: web::Data<Connections>,
    category: web::Path<String>,
) 
-> Result<HttpResponse, crate::AppError> 
{
    let mut categories = String::from("");
    let split_categories: Vec<&str> = category.split('-').collect();
    for (c_index, category) in split_categories.iter().enumerate() {
        if c_index != split_categories.len() - 1 {
            categories.push_str(&format!("'{}',", category));
        } else {
            categories.push_str(&format!("'{}'", category));
        }
    }

    let query = format!("{} IN ({})", BOOKS_QUERY_CATEGORY, categories);
    let query = Query::new(query).with_page_size(GET_SIZE);
    let documents = app.query(query, &[])
    .await?;
    let page = documents.paging_state.clone();
    let documents: Option<Vec<CategoryBookMetadata>> = documents.get_query_result()?;
    match documents {
        Some(docs) => {
            let page = match page {
                Some(page) => Some(page.to_vec()),
                None => None,
            };
            Ok(HttpResponse::Ok().json(CategoryBooksMetadataResponse{books: docs, page }))
        },
        None => {
            let x: Vec<CategoryBookMetadata> = Vec::new();
            let y = CategoryBooksMetadataResponse{books: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}


#[derive(FromRow, Serialize)]
pub struct CategoryBookMetadata {
    category: String,
    bookId: Uuid,
    authorId: i32,
    title: String,
    body: String,
    url: Option<String>,
    metadata: String,
    createdAt: Uuid,
    updatedAt: Uuid,
}

#[derive(Serialize)]
pub struct CategoryBooksMetadataResponse {
    books: Vec<CategoryBookMetadata>,
    page: Option<Vec<u8>>,
}


pub async fn getBooksWithPageSizeCategoriesNext(
    app: web::Data<Connections>,
    category: web::Path<String>,
    request: web::Json<NextPageRequest>,
) 
-> Result<HttpResponse, crate::AppError> 
{
    let mut categories = String::from("");
    let split_categories: Vec<&str> = category.split('-').collect();
    for (c_index, category) in split_categories.iter().enumerate() {
        if c_index != split_categories.len() - 1 {
            categories.push_str(&format!("'{}',", category));
        } else {
            categories.push_str(&format!("'{}'", category));
        }
    }

    let query = format!("{} IN ({})", BOOKS_QUERY_CATEGORY, categories);
    let query = Query::new(query).with_page_size(GET_SIZE);
    let documents = app.query_paged(query, &[], request.page.clone())
    .await?;
    let page = documents.paging_state.clone();
    let documents: Option<Vec<CategoryBookMetadata>> = documents.get_query_result()?;
    match documents {
        Some(docs) => {
            let page = match page {
                Some(page) => Some(page.to_vec()),
                None => None,
            };
            Ok(HttpResponse::Ok().json(CategoryBooksMetadataResponse{books: docs, page }))
        },
        None => {
            let x: Vec<CategoryBookMetadata> = Vec::new();
            let y = CategoryBooksMetadataResponse{books: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}