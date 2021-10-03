use actix_web::{HttpResponse, web};
use scylla::batch::Batch;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::App;
use validator::Validate;
use lily_utils::time_uuid;
use scylla::frame::response::cql_to_rust::FromRow;
use scylla::macros::FromRow;
use crate::utils::{
	ConnectionResult
};
use crate::AppError;
use crate::book::queries::{UPDATE_PARENT_ID, CHILD};

#[derive(Deserialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct Request {
    title: String,
    body: String,
    identity: i16,
    bookId: String,
    topUniqueId: String,
    botUniqueId: String,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct Response {
    uniqueId: String,
}

pub async fn create_and_update_chapter(
    _app: web::Data<App>, 
    payload: web::Json<Request>
) 
-> Result<HttpResponse, actix_web::Error> 
{
    // init
    let conn = _app.conn_result()?;
    
    // query
    let mut batch: Batch = Default::default();
    batch.append_statement(UPDATE_PARENT_ID);
    batch.append_statement(CHILD);
    
    // init ids
    let new_id = time_uuid();
    let book_id = Uuid::parse_str(&payload.bookId).unwrap();
    let unique_id = Uuid::parse_str(&payload.botUniqueId).unwrap();
    let parent_id = Uuid::parse_str(&payload.topUniqueId).unwrap();
    
    // query values
    let batch_values = (
        (&new_id, book_id.clone(), &unique_id),                
        (book_id,&new_id,&parent_id, &payload.title, &payload.body, &payload.identity,&new_id,&new_id)
    );
    match conn.batch(&batch, batch_values).await {
        Ok(_) => Ok(HttpResponse::Ok().json(Response {
            uniqueId: unique_id.to_string()
        })),
        Err(err) => Err(AppError::from(err).into())
    }
}