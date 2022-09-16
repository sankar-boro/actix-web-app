use actix_web::{HttpResponse,web};
use crate::App;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use scylla::{macros::FromRow, query::Query};
use crate::utils::{
    ParseUuid,
	GetQueryResult
};

#[derive(FromRow, Serialize)]
pub struct BlogMetadata {
    blogId: Uuid,
    authorId: Uuid,
    title: String,
    body: String,
    url: Option<String>,
    metadata: String,
    createdAt: Uuid,
    updatedAt: Uuid,
}

#[derive(Serialize)]
pub struct BlogsMetadataResponse {
    blogs: Vec<BlogMetadata>,
    page: Option<Vec<u8>>,
}

// cannot use * when getting all documents;
static BLOGS_QUERY: &'static str = "SELECT blogId, authorId, title, body, url, metadata, createdAt, updatedAt from sankar.blogs";
pub async fn getBlogsWithPageSize(
    app: web::Data<App>
) 
-> Result<HttpResponse, crate::AppError> {
    let query = Query::new(BLOGS_QUERY).with_page_size(4);
    let documents = 
    app.query(query, &[])
    .await?;
    let page = documents.paging_state.clone();
    let documents: Option<Vec<BlogMetadata>> = documents.get_query_result()?;
    match documents {
        Some(docs) => {
            let page = match page {
                Some(page) => Some(page.to_vec()),
                None => None,
            };
            Ok(HttpResponse::Ok().json(BlogsMetadataResponse{
                blogs: docs,
                page
            }))
        },
        None => {
            let x: Vec<BlogMetadata> = Vec::new();
            let y = BlogsMetadataResponse{blogs: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}

#[derive(Serialize, Deserialize)]
pub struct NextPageRequest {
    page: Vec<u8>
}

// cannot use * when getting all documents;
pub async fn getNextBlogsWithPageSize(
    app: web::Data<App>, 
    request: web::Json<NextPageRequest>
) 
-> Result<HttpResponse, crate::AppError> {
    let query = Query::new(BLOGS_QUERY).with_page_size(4);
    let documents = 
    app.query_paged(query, &[], request.page.clone())
    .await?;
    let page = documents.paging_state.clone();
    let documents: Option<Vec<BlogMetadata>> = documents.get_query_result()?;
    match documents {
        Some(blogs) => {
            let page = match page {
                Some(page) => Some(page.to_vec()),
                None => None,
            };
            Ok(HttpResponse::Ok().json(BlogsMetadataResponse{
                blogs,
                page
            }))
        },
        None => {
            let x: Vec<BlogMetadata> = Vec::new();
            let y = BlogsMetadataResponse{blogs: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}

#[derive(FromRow, Serialize)]
pub struct BlogNode {
    blogId: Uuid,
    uniqueId: Uuid,
    parentId: Option<Uuid>,
    authorId: Option<Uuid>,
    title: String,
    body: String,
    identity: i16,
    url: Option<String>,
    createdAt: Uuid,
    updatedAt: Uuid,
}

#[derive(Serialize)]
pub struct BlogNodesResponse {
    nodes: Vec<BlogNode>,
    page: Option<Vec<u8>>,
}

static GET_BLOG_NODES_WITH_PAGE_SIZE: &'static str = "SELECT blogId, uniqueId, parentId, authorId, title, body, identity, url, createdAt, updatedAt from sankar.blog WHERE blogId=";
pub async fn getBlogNodesWithPageSizeFromId(
    app: web::Data<App>, 
    blog_id: web::Path<String>
) -> Result<HttpResponse, crate::AppError> 
{
    let query = format!("{}{}", GET_BLOG_NODES_WITH_PAGE_SIZE, &blog_id.to_uuid()?);
    let query = Query::new(query).with_page_size(3);
    let documents = 
		app.query(query, &[]).await?;
    let page = documents.paging_state.clone();
	let documents: Option<Vec<BlogNode>> = documents.get_query_result()?;

    match documents {
        Some(nodes) => {
            let page = match page {
                Some(page) => {
                    Some(page.to_vec())
                },
                None => None,
            };
            Ok(HttpResponse::Ok().json(BlogNodesResponse{nodes, page}))
        },
        None => {
            let x: Vec<BlogNode> = Vec::new();
            let y = BlogNodesResponse{nodes: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}


pub async fn getNextBlogNodesWithPageSizeFromId(
    app: web::Data<App>, 
    blog_id: web::Path<String>,
    request: web::Json<NextPageRequest>,
) -> Result<HttpResponse, crate::AppError> {
    let query = format!("{}{}", GET_BLOG_NODES_WITH_PAGE_SIZE, &blog_id.to_uuid()?);
    let query = Query::new(query).with_page_size(3);
    let documents = 
		app.query_paged(query, &[], request.page.clone())
		.await?;
    let page = documents.paging_state.clone();
    let documents: Option<Vec<BlogNode>> = documents.get_query_result()?;

    match documents {
        Some(nodes) => {
            let page = match page {
                Some(page) => {
                    Some(page.to_vec())
                },
                None => None,
            };
            return Ok(HttpResponse::Ok().json(BlogNodesResponse{nodes, page }));
        },
        None => {
            let x: Vec<BlogNode> = Vec::new();
            let y = BlogNodesResponse{nodes: x, page: None };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}