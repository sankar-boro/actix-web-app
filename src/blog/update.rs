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
    blogId: String,
    uniqueId: String,
}

pub async fn update(
    app: web::Data<App>, 
    payload: web::Json<UpdateRequest>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let blogId = Uuid::parse_str(&payload.blogId)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;

    let mut batch: Batch = Default::default();

    let blogQuery = Query::from(format!("UPDATE sankar.blog SET title=?, body=? WHERE blogId=? AND uniqueId=?"));
    batch.append_statement(blogQuery);

    let blogQueryInfo = Query::from(format!("UPDATE sankar.blogInfo SET title=?, body=? WHERE blogId=?"));
    batch.append_statement(blogQueryInfo);


    app.batch(&batch, (
            (&payload.title, &payload.body, &blogId, &uniqueId),
            (&payload.title, &payload.body, &blogId),
        )
    ).await?;
    Ok(HttpResponse::Ok().body("Updated".to_string()))
}
