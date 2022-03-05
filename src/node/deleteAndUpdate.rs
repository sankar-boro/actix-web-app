use uuid::Uuid;
use crate::App;
use serde::Deserialize;
use actix_web::{web, HttpResponse};
use scylla::batch::Batch;
use scylla::query::Query;

#[derive(Deserialize)]
struct UpdateData {
    topUniqueId: String,
    botUniqueId: String,
}

#[derive(Deserialize)]
pub struct UpdateOrDelete {
    bookId: String,
    updateData: UpdateData,
    deleteData: Vec<String>,
}

pub async fn deleteAndUpdate(
    app: web::Data<App>, 
    payload: web::Json<UpdateOrDelete>
) -> Result<HttpResponse, crate::AppError> {

    let update_data = &payload.updateData;
    let book_id = Uuid::parse_str(&payload.bookId)?;

    let mut batch: Batch = Default::default();

    // update query
    let query = Query::new(
        format!("UPDATE sankar.book SET parentId={} WHERE bookId={} AND uniqueId={}", 
        &update_data.topUniqueId, 
        &book_id, 
        &update_data.botUniqueId)
    );
    batch.append_statement(query); // append query

    // delete query
    let deleteData = &payload.deleteData;
    let mut deleteData = deleteData.iter();
    let mut uniqueIds = String::from("");
    if let Some(id) = deleteData.next() {
        uniqueIds.push_str(id);
    }
    while let Some(id) = deleteData.next() {
        uniqueIds.push_str(&format!(", {}", &id));
    }
    let query = Query::new(format!(
        "DELETE FROM sankar.book WHERE bookId={} AND uniqueId IN ({})",
        &book_id,
        &uniqueIds)
    );
    batch.append_statement(query); // append query

    app.batch(&batch, ((), ())).await?;
    Ok(HttpResponse::Ok().body("Updated or deleted."))
}
