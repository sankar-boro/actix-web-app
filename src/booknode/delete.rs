use uuid::Uuid;
use crate::App;
use serde::Deserialize;
use actix_web::{web, HttpResponse};

#[derive(Deserialize)]
pub struct DeleteNodeRequest {
    bookId: String,
    deleteData: Vec<String>,
}

pub async fn delete(
    app: web::Data<App>, 
    payload: web::Json<DeleteNodeRequest>
) -> Result<HttpResponse, crate::AppError> {
    let book_id = Uuid::parse_str(&payload.bookId)?;
    let deleteData = &payload.deleteData;
    let mut deleteData = deleteData.iter();
    
    let mut uniqueIds = String::from("");
    if let Some(id) = deleteData.next() {
        uniqueIds.push_str(id);
    }
    while let Some(id) = deleteData.next() {
        uniqueIds.push_str(&format!(", {}", &id));
    }

    let query = format!("DELETE FROM sankar.book WHERE bookId={} AND uniqueId IN ({})", &book_id, &uniqueIds);

    app.query(query, &[]).await?;
    Ok(HttpResponse::Ok().body("Deleted."))
}
