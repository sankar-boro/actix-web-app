use uuid::Uuid;
use crate::App;
use serde::Deserialize;
use validator::Validate;
use scylla::macros::FromRow;
use actix_web::{web, HttpResponse};
use crate::utils::{ConnectionResult};
use scylla::frame::response::cql_to_rust::FromRow;
use scylla::batch::Batch;
use crate::AppError;

#[derive(Deserialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct Delete {
    bookId: String,
    uniqueId: String,
}

pub async fn delete_section_last(
    session: web::Data<App>, 
    payload: web::Json<Delete>
)
-> Result<HttpResponse, actix_web::Error> 
{
    let conn = session.conn_result()?;
    let book_id =  Uuid::parse_str(&payload.bookId).unwrap();
    let unique_id =  Uuid::parse_str(&payload.uniqueId).unwrap();
    conn
    .query("DELETE FROM sankar.book WHERE bookId=? AND uniqueId=?", (book_id, unique_id))
    .await.unwrap();
    Ok(HttpResponse::Ok().body("Document deleted"))
}

#[derive(Deserialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct PayloadUpdateAndDelete {
    data: String,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct UpdateData {
    bookId: String,
    uniqueId: String,
    newParentId: String,
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct DeleteData {
    bookId: String,
    deleteUniqueId: String,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct UpdateAndDelete {
    updateData: UpdateData,
    deleteData: DeleteData,
}

pub async fn delete_section_first(
    session: web::Data<App>, 
    payload: web::Json<PayloadUpdateAndDelete>
)
-> Result<HttpResponse, actix_web::Error> 
{
    println!("{}", payload.data);

    let conn = session.conn_result()?;
    let p: UpdateAndDelete = serde_json::from_str(&payload.data).unwrap();
    let u = p.updateData;
    let d = p.deleteData;

    let mut batch: Batch = Default::default();
    batch.append_statement("UPDATE sankar.book SET parentId=? WHERE bookId=? AND uniqueId=?");
    batch.append_statement("DELETE FROM sankar.book WHERE bookId=? AND uniqueId=?");

    let book_id = Uuid::parse_str(&u.bookId).unwrap();
    let new_parent_id = Uuid::parse_str(&u.newParentId).unwrap();
    let unique_id = Uuid::parse_str(&u.uniqueId).unwrap();
    let delete_id = Uuid::parse_str(&d.deleteUniqueId).unwrap();
    let batch_values = (
        (&new_parent_id, &book_id, &unique_id),                
        (&book_id, &delete_id)
    );

    match conn.batch(&batch, batch_values).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Updated and created new chapter.")),
        Err(err) => Err(AppError::from(err).into())
    }
}
