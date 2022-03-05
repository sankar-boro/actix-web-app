use uuid::Uuid;
use crate::App;
use actix_web::{web, HttpResponse};
use scylla::batch::Batch;

pub static DELETE_BOOK: &str = "DELETE FROM sankar.book where bookId=?";
pub static DELETE_BOOK_INFO: &str = "DELETE FROM sankar.bookInfo where bookId=?";
pub async fn delete(
    app: web::Data<App>,
    bookId: web::Path<String>
) -> Result<HttpResponse, crate::AppError> {
    let book_id = Uuid::parse_str(&bookId)?;

    let mut batch: Batch = Default::default();
    batch.append_statement(DELETE_BOOK);
    batch.append_statement(DELETE_BOOK_INFO);
    
    let batch_values = ((&book_id,), (&book_id,));
    app.batch(&batch, &batch_values).await?;
    Ok(HttpResponse::Ok().body("Deleted book."))
}