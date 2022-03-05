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
    let delete_data = &payload.deleteData;
    let book_id = Uuid::parse_str(&payload.bookId)?;

    let mut batch: Batch = Default::default();

    // update query
    let update_query = format!("UPDATE sankar.book SET parentId={} WHERE bookId={} AND uniqueId={}", &update_data.topUniqueId, &book_id, &update_data.botUniqueId);
    let query: Query = Query::new(update_query);
    batch.append_statement(query);
    //

    // delete query
    if delete_data.len() > 0 {
        let mut delete_query = format!("DELETE FROM sankar.book WHERE bookId={} AND uniqueId IN (", &book_id);
        for (_i, del_item) in delete_data.iter().enumerate() {
            if _i == 0 {
                delete_query.push_str(&del_item);
            } else {
                delete_query.push_str(&format!(", {}", &del_item));    
            }
        }
        delete_query.push_str(")");
        batch.append_statement(Query::new(delete_query));
    }
    //

    app.batch(&batch, ((), ())).await?;
    Ok(HttpResponse::Ok().body("Updated or deleted."))
}
