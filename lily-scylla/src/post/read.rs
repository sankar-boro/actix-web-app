use actix_web::{HttpResponse,web};
use crate::App;
use uuid::Uuid;
use serde::Serialize;

use scylla::IntoTypedRows;
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;
use crate::RequestError;

#[derive(FromRow, Serialize)]
#[allow(non_snake_case)]
pub struct NewDocument {
    documentId: Uuid,
    title: String,
    tags: String,
    body: String,
    smDocumentImageUrl: Option<String>,
    mdDocumentImageUrl: Option<String>,
    lgDocumentImageUrl: Option<String>,
    authorId: Uuid,
    authorName: Option<String>,
    authorImageUrl: Option<String>,
    createdAt: Uuid,
    updatedAt: Uuid,
}

static GET_ALL: &'static str = "SELECT documentId, title, tags, body, authorId, createdAt, updatedAt from sankar.documents";
static GET_ONE: &'static str = "SELECT documentId, title, tags, body, authorId, createdAt, updatedAt from sankar.documents WHERE documentId={} LIMIT 1";

pub async fn get_all(session: web::Data<App>) -> HttpResponse {
    let conn = session.as_ref().conn();

    if let Ok(conn) = conn {
        if let Some(rows) = conn.query(GET_ALL, &[]).await.unwrap().rows {
            let mut documents = Vec::new();
            for row in rows.into_typed::<NewDocument>() {
                let new_document: NewDocument = row.unwrap();
                documents.push(new_document);
            }
            return HttpResponse::Ok().json(documents);
        }
        let error = RequestError::not_found("User not found.");
        return HttpResponse::Ok().json(error);
    }
    let error = RequestError::db_error("Something went wrong with DB.");
    HttpResponse::Ok().json(error)
}

pub async fn get_one(session: web::Data<App>, id: web::Path<String>,) -> HttpResponse {
    let conn = session.session.get().unwrap();
    let document_id =  Uuid::parse_str(&id).unwrap();
    if let Some(rows) = conn.query(format!("{} {}", GET_ONE, document_id), &[]).await.unwrap().rows {
        let mut documents = Vec::new();
        for row in rows.into_typed::<NewDocument>() {
            let new_document: NewDocument = row.unwrap();
            documents.push(new_document);
        }
        return HttpResponse::Ok().json(documents);
    }
    HttpResponse::Ok().body("Failed to get document")
}