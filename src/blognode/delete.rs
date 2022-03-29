use uuid::Uuid;
use crate::App;
use serde::Deserialize;
use actix_web::{web, HttpResponse};

#[derive(Deserialize)]
pub struct DeleteNodeRequest {
    blogId: String,
    deleteData: Vec<String>,
}

pub async fn delete(
    app: web::Data<App>, 
    payload: web::Json<DeleteNodeRequest>
) -> Result<HttpResponse, crate::AppError> {
    let blog_id = Uuid::parse_str(&payload.blogId)?;
    let deleteData = &payload.deleteData;
    let mut deleteData = deleteData.iter();
    
    let mut uniqueIds = String::from("");
    if let Some(id) = deleteData.next() {
        uniqueIds.push_str(id);
    }
    while let Some(id) = deleteData.next() {
        uniqueIds.push_str(&format!(", {}", &id));
    }

    let query = format!("DELETE FROM sankar.blog WHERE blogId={} AND uniqueId IN ({})", &blog_id, &uniqueIds);

    app.query(query, &[]).await?;
    Ok(HttpResponse::Ok().body("Deleted."))
}
