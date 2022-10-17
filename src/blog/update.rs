use actix_web::{HttpResponse, web};
use serde::Deserialize;
use crate::App;
use scylla::macros::FromRow;
use scylla::batch::Batch;
use scylla::query::Query;
use uuid::Uuid;
use actix_session::Session;
use crate::auth::AuthSession;

#[derive(Deserialize, FromRow)]
pub struct UpdateRequest {
    title: String,
    body: String,
    blogId: String,
    uniqueId: String,
    category: String,
    metadata: String,
}

pub async fn update(
    app: web::Data<App>, 
    payload: web::Json<UpdateRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let blogId = Uuid::parse_str(&payload.blogId)?;
    let uniqueId = Uuid::parse_str(&payload.uniqueId)?;
    let auth = session.user_info()?;
    let auth_id = Uuid::parse_str(&auth.userId)?;

    let mut batch: Batch = Default::default();
    let blogQuery = Query::from(format!("UPDATE sankar.blog SET title=?, body=?, metadata=? WHERE blogId=? AND uniqueId=?"));
    let blogsQuery = Query::from(format!("UPDATE sankar.blogs SET title=?, body=?, metadata=? WHERE blogId=?"));
    let userBlogsQuery = Query::from(format!("UPDATE sankar.userblogs SET title=?, body=?, metadata=? WHERE authorId=? AND blogId=?"));
    let categoryBlogsQuery = Query::from(format!("UPDATE sankar.categoryblogs SET title=?, body=?, metadata=? WHERE category=? AND blogId=?"));

    batch.append_statement(blogQuery);
    batch.append_statement(blogsQuery);
    batch.append_statement(userBlogsQuery);
    batch.append_statement(categoryBlogsQuery);
    app.batch(&batch, (
        (&payload.title, &payload.body, &payload.metadata, &blogId, &uniqueId),
        (&payload.title, &payload.body, &payload.metadata, &blogId),
        (&payload.title, &payload.body, &payload.metadata, &auth_id, &blogId),
        (&payload.title, &payload.body, &payload.metadata, &payload.category, &blogId),
    )).await?;

    Ok(HttpResponse::Ok().body("Updated".to_string()))
}
