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
    let delete_data = &payload.deleteData;

    // make delete query
    let mut delete_query = format!("DELETE FROM sankar.book WHERE bookId={} AND uniqueId IN (", &book_id);
    for (_i, del_item) in delete_data.iter().enumerate() {
        if _i == 0 {
            delete_query.push_str(&del_item);
        } else {
            delete_query.push_str(&format!(", {}", &del_item));    
        }
    }
    delete_query.push_str(")");
    //

    app.query(delete_query, &[]).await?;
    Ok(HttpResponse::Ok().body("Deleted."))
}
