use actix_web::get;
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

static GET_ALL_DOCUMENTS: &'static str = "SELECT documentId, title, tags, body, authorId, createdAt, updatedAt from sankar.documents";
static GET_DOCUMENT: &'static str = "SELECT documentId, title, tags, body, authorId, createdAt, updatedAt from sankar.documents WHERE documentId={} LIMIT 1";

fn res_err(err: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(RequestError::db_error(&err.to_string()))
}

#[get("/all")]
pub async fn get_all(_app: web::Data<App>) -> HttpResponse {
    let conn = match _app.as_ref().conn() {
        Ok(conn) => conn,
        Err(err) => return res_err(&err.to_string()),
    };

    let documents = match conn.query(GET_ALL_DOCUMENTS, &[]).await {
        Ok(docs) => docs,
        Err(err) => return res_err(&err.to_string()),
    };

    // TODO: should recover from unwrap()
    let documents = match documents.rows {
        Some(docs) => docs.into_typed::<NewDocument>().map(|a| a.unwrap()).collect::<Vec<NewDocument>>(),
        None => {
            let res: Vec<NewDocument> = Vec::new();
            return HttpResponse::Ok().json(res);
        },
    };

    HttpResponse::Ok().json(documents)
}

pub async fn get_one(session: web::Data<App>, id: web::Path<String>,) -> HttpResponse {
    let conn = match session.as_ref().conn() {
        Ok(conn) => conn,
        Err(err) => return res_err(&err.to_string()),
    };

    let document_id =  match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(err) => return res_err(&err.to_string()) 
    };
    

    let documents = match conn.query(format!("{} {}", GET_DOCUMENT, document_id), &[]).await {
        Ok(docs) => docs,
        Err(err) => return res_err(&err.to_string()),
    };

    // TODO: should recover from unwrap()
    let documents = match documents.rows {
        Some(docs) => docs.into_typed::<NewDocument>().map(|a| a.unwrap()).collect::<Vec<NewDocument>>(),
        None => {
            let res: Vec<NewDocument> = Vec::new();
            return HttpResponse::Ok().json(res);
        },
    };

    HttpResponse::Ok().json(documents)
}