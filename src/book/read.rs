use actix_web::{HttpResponse,web};
use crate::App;
use uuid::Uuid;
use serde::Serialize;

use scylla::macros::FromRow;
use crate::utils::{
	GetQueryResult
};
use crate::AppError;

#[derive(FromRow, Serialize)]
#[allow(non_snake_case)]
pub struct NewDocument {
    documentId: Uuid,
    title: String,
    tags: String,
    body: String
}

#[derive(FromRow, Serialize)]
#[allow(non_snake_case)]
pub struct Book {
    bookId: Uuid,
    uniqueId: Uuid,
    parentId: Option<Uuid>,
    authorId: Option<Uuid>,
    authorName: Option<String>,
    title: String,
    body: String,
    identity: i16,
    createdAt: Uuid,
    updatedAt: Uuid,
}

static GET_ALL_DOCUMENTS: &'static str = "SELECT bookId, uniqueId, parentId, authorId, authorName, title, body, identity, createdAt, updatedAt from sankar.book";
static GET_ALL_DOCUMENTS_FROM_ID: &'static str = "SELECT bookId, uniqueId, parentId, authorId, authorName, title, body, identity, createdAt, updatedAt from sankar.book WHERE bookId=";

pub async fn get_all(app: web::Data<App>) 
-> Result<HttpResponse, actix_web::Error> {
    let documents: Option<Vec<Book>> = 
    app.session.query(GET_ALL_DOCUMENTS, &[])
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

fn get_document_query(document_id: &str) 
-> Result<String, actix_web::Error> {
    match Uuid::parse_str(document_id) {
        Ok(document_id) => {
            Ok(format!("{}{}", GET_ALL_DOCUMENTS_FROM_ID, document_id))
        }
        Err(err) => Err(AppError::from(err).into())
    }
}

fn get_all_document_from_id_query(document_id: &str) 
-> Result<String, actix_web::Error> {
    match Uuid::parse_str(document_id) {
        Ok(document_id) => {
            let q = format!("{}{}", GET_ALL_DOCUMENTS_FROM_ID, document_id);
            Ok(q)
        }
        Err(err) => Err(AppError::from(err).into())
    }
}

pub async fn get_one(app: web::Data<App>, document_id: web::Path<String>,) 
-> Result<HttpResponse, actix_web::Error> {

    let documents: Option<Vec<Book>> = 
		app.session.query(get_document_query(&document_id)?, &[])
		.await
		.get_query_result()?;

    // TODO: should recover from unwrap()
    match documents {
        Some(docs) => Ok(HttpResponse::Ok().json(docs)),
        None => {
            let mt: Vec<Book> = Vec::new();
            Ok(HttpResponse::Ok().json(mt))
        },
    }
}

pub async fn get_all_from_id(app: web::Data<App>, document_id: web::Path<String>) 
-> Result<HttpResponse, actix_web::Error> {
    let _docs = get_all_document_from_id_query(&document_id)?;
    let documents: Option<Vec<Book>> = 
		app.session.query(_docs, &[])
		.await
		.get_query_result()?;

    // TODO: should recover from unwrap()
    match documents {
        Some(docs) => Ok(HttpResponse::Ok().json(docs)),
        None => {
            let mt: Vec<Book> = Vec::new();
            Ok(HttpResponse::Ok().json(mt))
        },
    }
}