use uuid::Uuid;
use crate::App;
use serde::Deserialize;
use validator::Validate;
use scylla::macros::FromRow;
use actix_web::{web, HttpResponse};
use crate::utils::{ConnectionResult};
use scylla::frame::response::cql_to_rust::FromRow;

#[derive(Deserialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct UpdateAndInsert {
    bookId: String,
    uniqueId: String,
}

pub async fn delete_section(
    session: web::Data<App>, 
    payload: web::Json<UpdateAndInsert>
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
