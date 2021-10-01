use actix_web::{HttpResponse, web};
use scylla::batch::Batch;
use serde::{Deserialize};
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
use crate::book::queries::{UPDATE, CHILD};



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

pub async fn create_and_update_chapter(
    _app: web::Data<App>, 
    payload: web::Json<Request>
) 
-> Result<HttpResponse, actix_web::Error> 
{
    let new_id = time_uuid();
    let conn = _app.conn_result()?;
    let mut batch: Batch = Default::default();
    batch.append_statement(UPDATE);
    batch.append_statement(CHILD);
    let bo_ok_id = Uuid::parse_str(&payload.bookId).unwrap();
    let u_id = Uuid::parse_str(&payload.botUniqueId).unwrap();
    let p_id = Uuid::parse_str(&payload.topUniqueId).unwrap();
    let batch_values = (
        (&new_id, bo_ok_id.clone(), &u_id),                
        (bo_ok_id,&new_id,&p_id, &payload.title, &payload.body, &payload.identity,&new_id,&new_id)
    );
    match conn.batch(&batch, batch_values).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Updated and created new chapter.")),
        Err(err) => Err(AppError::from(err).into())
    }
}