use actix_session::Session;
use actix_web::dev::ServiceRequest;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize};
use uuid::Uuid;
use crate::App;
use validator::Validate;
use lily_utils::time_uuid;
use scylla::frame::response::cql_to_rust::FromRow;
use scylla::macros::FromRow;
use crate::utils::{
	GetQueryResult, 
	ConnectionResult
};
use serde::Serialize;


#[derive(Deserialize, Validate, FromRow)]
pub struct NewDocumentForm {
    title: String,
    tags: String,
    body: String,
    authorId: String
}

#[derive(Serialize, Validate, FromRow)]
pub struct DocumentResponse {
    documentId: Uuid,
    title: String,
    tags: String,
    body: String,
    authorId: Uuid,
}

static INSERT_DOCUMENT_INTO__DOCUMENTS: &str = "INSERT INTO sankar.documents (documentId,title,tags,body,authorId) VALUES(?,?,?,?,?)";

pub async fn create_one(
    _app: web::Data<App>, 
    request: web::Json<NewDocumentForm>, 
) 
-> Result<HttpResponse, actix_web::Error> 
{
    let conn = _app.conn_result()?;    
    let doc_id = time_uuid();
    let _: Option<Vec<NewDocumentForm>> = conn
        .query(INSERT_DOCUMENT_INTO__DOCUMENTS, 
            (doc_id, &request.title, &request.tags,&request.body, Uuid::parse_str(&request.authorId).unwrap())
        ).await.get_query_result()?;
    let res = DocumentResponse { 
        documentId: doc_id.clone(), 
        title: request.title.to_string(),
        tags: request.tags.to_string(),
        body: request.body.to_string(),
        authorId: doc_id
    };
    Ok(
        HttpResponse::Ok().json(res)
    )
}

// HELP
// When creating query, check for commas also. They might cause issue. Right now its working.
