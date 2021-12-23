use uuid::Uuid;
use crate::App;
use serde::Deserialize;
use actix_web::{web, HttpResponse};
use crate::utils::{ConnectionResult};
use scylla::batch::Batch;
use crate::AppError;
use scylla::query::Query;

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct UpdateData {
    uniqueId: String,
    newParentId: String,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct UpdateOrDeleteInner {
    updateData: Option<UpdateData>,
    deleteData: Vec<String>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct UpdateOrDelete {
    bookId: String,
    json: String,
}

pub async fn update_or_delete(
    session: web::Data<App>, 
    payload: web::Json<UpdateOrDelete>
) -> Result<HttpResponse, actix_web::Error> {
    
    let conn = session.conn_result()?;
    let _json: UpdateOrDeleteInner = serde_json::from_str(&payload.json).unwrap();

    let update_data = _json.updateData;
    let delete_data = _json.deleteData;
    let book_id = Uuid::parse_str(&payload.bookId).unwrap();

    let mut batch: Batch = Default::default();

    if let Some(update_data) = &update_data {
        let update_query = format!("UPDATE sankar.book SET parentId={} WHERE bookId={} AND uniqueId={}", &update_data.newParentId, &book_id, &update_data.uniqueId);
        log::info!("{}", &update_query);
        let query: Query = Query::new(update_query);
        batch.append_statement(query);
    }

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
        log::info!("{}", delete_query);
        batch.append_statement(Query::new(delete_query));
    }

    if let Some(_) = &update_data {
        return match conn.batch(&batch, ((), ())).await {
            Ok(_) => Ok(HttpResponse::Ok().body("Updated or deleted.")),
            Err(err) => Err(AppError::from(err).into())
        }
    } else {
        return match conn.batch(&batch, ((),)).await {
            Ok(_) => Ok(HttpResponse::Ok().body("Updated or deleted.")),
            Err(err) => Err(AppError::from(err).into())
        }
    }
}