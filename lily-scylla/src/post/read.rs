use actix_web::get;
use actix_web::{HttpResponse,web};
use crate::App;
use uuid::Uuid;
use serde::Serialize;

use scylla::IntoTypedRows;
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;
use crate::RequestError;
use crate::utils::{
	GetQueryResult, 
	ConnectionResult
};
use crate::AppError;

#[derive(FromRow, Serialize)]
#[allow(non_snake_case)]
pub struct NewDocument {
    documentId: Uuid,
    title: String,
    tags: String,
    body: String,
    // smDocumentImageUrl: Option<String>,
    // mdDocumentImageUrl: Option<String>,
    // lgDocumentImageUrl: Option<String>,
    // authorId: Uuid,
    // authorName: Option<String>,
    // authorImageUrl: Option<String>,
    // createdAt: Uuid,
    // updatedAt: Uuid,
}

#[derive(FromRow, Serialize)]
#[allow(non_snake_case)]
pub struct DocumentResponse {
    documentId: Uuid,
    title: String,
    tags: String,
    body: String,
    authorId: Uuid,
}

static GET_ALL_DOCUMENTS: &'static str = "SELECT documentId, title, tags, body, authorId from sankar.documents";
static GET_DOCUMENT: &'static str = "SELECT documentId, title, tags, body, authorId from sankar.documents WHERE documentId={} LIMIT 1";

pub async fn get_all(_app: web::Data<App>) 
-> Result<HttpResponse, actix_web::Error> {
    let conn = _app.conn_result()?;

    let documents: Option<Vec<DocumentResponse>> = 
		conn.query(GET_ALL_DOCUMENTS, &[])
		.await
		.get_query_result()?;

    match documents {
        Some(docs) => Ok(HttpResponse::Ok().json(docs)),
        None => {
            let mt: Vec<DocumentResponse> = Vec::new();
            Ok(HttpResponse::Ok().json(mt))
        },
    }
}

fn get_document_query(document_id: &str) 
-> Result<String, actix_web::Error> {
    match Uuid::parse_str(document_id) {
        Ok(document_id) => {
            Ok(format!("{} {}", GET_DOCUMENT, document_id))
        }
        Err(err) => Err(AppError::from(err).into())
    }
}

pub async fn get_one(session: web::Data<App>, document_id: web::Path<String>,) 
-> Result<HttpResponse, actix_web::Error> {
    let conn = session.conn_result()?;

    let documents: Option<Vec<DocumentResponse>> = 
		conn.query(get_document_query(&document_id)?, &[])
		.await
		.get_query_result()?;

    // TODO: should recover from unwrap()
    match documents {
        Some(docs) => Ok(HttpResponse::Ok().json(docs)),
        None => {
            let mt: Vec<DocumentResponse> = Vec::new();
            Ok(HttpResponse::Ok().json(mt))
        },
    }
}