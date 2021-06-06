use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, web};
use rand::Rng;
use serde::{Deserialize};
use uuid::{Uuid, v1::{Timestamp, Context}};
use crate::App;
use validator::Validate;
use lily_service::encrypt_text;
use chrono::prelude::*;


#[derive(Deserialize, Validate)]
pub struct NewDocument {
    title: String,
    tags: String,
    body: String
}

fn uuid_new() -> Uuid { 
    let mut rng = rand::thread_rng();
    let rand: [u8; 6] = rng.gen();
    let rand_num: u16 = rng.gen();
    let context = Context::new(rand_num);
    let utc: DateTime<Utc> = Utc::now(); 
    let ts = Timestamp::from_unix(&context, utc.timestamp() as u64, utc.timestamp_subsec_nanos());
    Uuid::new_v1(ts, &rand).expect("failed to generate UUID")
}

pub async fn create_one(_app: web::Data<App>, request: web::Form<NewDocument>) -> HttpResponse {
    let conn = _app.get_ref().conn();
    if let Ok(conn) = conn {
        let doc_id = uuid_new();
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
