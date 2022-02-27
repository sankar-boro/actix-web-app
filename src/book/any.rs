use actix_web::{HttpResponse, web};
use scylla::batch::Batch;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::App;
use validator::Validate;
use lily_utils::time_uuid;
use scylla::macros::FromRow;
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
    botUniqueId: Option<String>,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct Response {
    uniqueId: String,
}

pub async fn any(
    app: web::Data<App>, 
    payload: web::Json<Request>
) 
-> Result<HttpResponse, actix_web::Error> 
{
    // query
    let mut batch: Batch = Default::default();
    batch.append_statement(UPDATE_PARENT_ID);
    
    // init ids
    let new_id = time_uuid();
    let book_id = Uuid::parse_str(&payload.bookId).unwrap();
    let top_unique_id = Uuid::parse_str(&payload.topUniqueId).unwrap();
    let unique_id = new_id.to_string();
    
    
    if let Some(bot_id) = &payload.botUniqueId {
        batch.append_statement(CHILD);
        let bot_unique_id = Uuid::parse_str(bot_id).unwrap();
        let batch_values = (
            (&new_id, book_id.clone(), &bot_unique_id), // update
            (book_id,&new_id,&top_unique_id, &payload.title, &payload.body, &payload.identity,&new_id,&new_id) // create
        );
        return match app.session.batch(&batch, batch_values).await {
            Ok(_) => Ok(HttpResponse::Ok().json(Response {
                uniqueId: unique_id
            })),
            Err(err) => Err(AppError::from(err).into())
        }
    }
    
    let batch_values = (book_id,&new_id,&top_unique_id, &payload.title, &payload.body, &payload.identity,&new_id,&new_id);
    let res = app.session
    .query(CHILD, batch_values)
    .await;

    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(Response {
            uniqueId: unique_id
        })),
        Err(err) => Err(AppError::from(err).into())
    }
}