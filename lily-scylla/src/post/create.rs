use actix_web::{HttpResponse, web};
use serde::{Deserialize};
use crate::App;
use validator::Validate;
use lily_utils::time_uuid;


#[derive(Deserialize, Validate)]
pub struct NewDocument {
    title: String,
    tags: String,
    body: String
}

pub async fn create_one(_app: web::Data<App>, request: web::Form<NewDocument>) -> HttpResponse {

    let conn = _app.get_ref().conn();
    if let Ok(conn) = conn {
        let doc_id = time_uuid();
        conn
            .query(
                "INSERT INTO sankar.documents (
                    documentId,
                    title,
                    tags, 
                    body
                ) VALUES(?,?,?,?)", 
                (
                    doc_id, 
                    &request.title, 
                    &request.tags, 
                    &request.body
                )
            ).await.unwrap();
    }
    HttpResponse::Ok().body("New document created!")
}

// HELP
// When creating query, check for commas also. They might cause issue. Right now its working.
