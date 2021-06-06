use actix_web::{HttpResponse, web};
use rand::Rng;
use serde::{Deserialize};
use uuid::{Uuid, v1::{Timestamp, Context}};
use crate::App;
use validator::Validate;
use chrono::prelude::*;
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use sanitize_filename;
use std::io::Write;


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

// NOTE: image wont upload from postman if you set Content-Type: multipart/form-data
pub async fn upload_image(mut payload: Multipart) -> HttpResponse {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || {
                let mut g = f.unwrap(); 
                g.write_all(&data).unwrap();
                Ok(g)
            }).await.unwrap();
        }
    }

    HttpResponse::Ok().body("Image uploaded!")
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
