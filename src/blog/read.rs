use actix_web::{HttpResponse,web};
use crate::App;
use uuid::Uuid;
use serde::Serialize;
use scylla::macros::FromRow;
use crate::utils::{
    ParseUuid,
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
pub struct BlogInfo {
    blogId: Uuid,
    authorId: Option<Uuid>,
    fname: Option<String>,
    lname: Option<String>,
    title: String,
    body: String,
    createdAt: Uuid,
    updatedAt: Uuid,
}

// cannot use * when getting all documents;
static GET_ALL_DOCUMENTS: &'static str = "SELECT blogId, authorId, fname, lname, title, body, createdAt, updatedAt from sankar.blogInfo";
pub async fn getAllBlogs(app: web::Data<App>) 
-> Result<HttpResponse, crate::AppError> {
    let documents: Option<Vec<BlogInfo>> = 
    app.query(GET_ALL_DOCUMENTS, &[])
    .await
    .get_query_result()?;
    match documents {
        Some(docs) => Ok(HttpResponse::Ok().json(docs)),
        None => {
            let mt: Vec<Blog> = Vec::new();
            Ok(HttpResponse::Ok().json(mt))
        },
    }
}

#[derive(FromRow, Serialize)]
pub struct Blog {
    blogId: Uuid,
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

static QUERY: &'static str = "SELECT blogId, uniqueId, parentId, authorId, fname, lname, title, body, identity, createdAt, updatedAt from sankar.blog WHERE blogId=";
pub async fn getAllNodesFromBlogId(app: web::Data<App>, blog_id: web::Path<String>) -> Result<HttpResponse, crate::AppError> {
    let query = format!("{}{}", QUERY, &blog_id.to_uuid()?);
    let documents: Option<Vec<Blog>> = 
		app.query(query, &[])
		.await
		.get_query_result()?;

    match documents {
        Some(docs) => Ok(HttpResponse::Ok().json(docs)),
        None => {
            let mt: Vec<Blog> = Vec::new();
            Ok(HttpResponse::Ok().json(mt))
        },
    }
}