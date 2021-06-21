use actix_web::{HttpResponse, web};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use sanitize_filename;
use std::io::Write;


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