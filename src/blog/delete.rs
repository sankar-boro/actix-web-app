use uuid::Uuid;
use crate::App;
use actix_web::{web, HttpResponse};
use scylla::batch::Batch;

pub static DELETE_BLOG: &str = "DELETE FROM sankar.blog where blogId=?";
pub static DELETE_BLOG_INFO: &str = "DELETE FROM sankar.blogInfo where blogId=?";

pub async fn delete(
    app: web::Data<App>,
    blogId: web::Path<String>
) -> Result<HttpResponse, crate::AppError> {
    let blog_id = Uuid::parse_str(&blogId)?;

    let mut batch: Batch = Default::default();
    batch.append_statement(DELETE_BLOG);
    batch.append_statement(DELETE_BLOG_INFO);
    
    let batch_values = ((&blog_id,), (&blog_id,));
    app.batch(&batch, &batch_values).await?;
    Ok(HttpResponse::Ok().body("Deleted blog."))
}