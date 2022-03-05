use actix_web::{HttpResponse, web};
use serde::Deserialize;
use crate::App;
use validator::Validate;
use scylla::macros::FromRow;
use scylla::batch::Batch;
use scylla::query::Query;
use uuid::Uuid;

#[derive(Deserialize, Validate, FromRow)]
pub struct UpdateRequest {
    title: String,
    body: String,
    bookId: String,
    uniqueId: String,
}

pub async fn update(
    app: web::Data<App>, 
    payload: web::Json<UpdateRequest>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let bookId = Uuid::parse_str(&payload.bookId)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;

    let mut batch: Batch = Default::default();

    let bookQuery = Query::from(format!("UPDATE sankar.book SET title=?, body=? WHERE bookId=? AND uniqueId=?"));
    batch.append_statement(bookQuery);

    let bookQueryInfo = Query::from(format!("UPDATE sankar.bookInfo SET title=?, body=? WHERE bookId=?"));
    batch.append_statement(bookQueryInfo);


    app.batch(&batch, (
            (&payload.title, &payload.body, &bookId, &uniqueId),
            (&payload.title, &payload.body, &bookId),
        )
    ).await?;
    Ok(HttpResponse::Ok().body("Updated".to_string()))
}
