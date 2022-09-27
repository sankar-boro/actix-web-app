use uuid::Uuid;
use crate::App;
use actix_web::{web, HttpResponse};
use scylla::batch::Batch;
use actix_session::Session;
use crate::auth::AuthSession;
use crate::utils::ParseUuid;

pub static DELETE_BOOKS: &str = "DELETE FROM sankar.books where bookId=?";
pub static DELETE_BOOK: &str = "DELETE FROM sankar.book where bookId=?";
pub static DELETE_USERBOOKS: &str = "DELETE FROM sankar.userbooks where authorId=? AND bookId IN (?)";
pub static DELETE_CATEGORYBOOKS: &str = "DELETE FROM sankar.categorybooks where category=? AND bookId IN (?)";

pub async fn delete(
    app: web::Data<App>,
    bookInfo: web::Path<(String, String)>,
    session: Session
) -> Result<HttpResponse, crate::AppError> {
    let book_id = Uuid::parse_str(&bookInfo.0)?;
    let category = &bookInfo.1;
    let auth = session.user_info()?;
    let auth_id = &auth.userId.to_uuid()?;

    let mut batch: Batch = Default::default();
    batch.append_statement(DELETE_BOOKS);
    batch.append_statement(DELETE_BOOK);
    batch.append_statement(DELETE_USERBOOKS);
    batch.append_statement(DELETE_CATEGORYBOOKS);
    
    let batch_values = ((&book_id,), (&book_id,), (&auth_id, &book_id,), (&category, &book_id,), );
    app.batch(&batch, &batch_values).await?;
    Ok(HttpResponse::Ok().body("Deleted book."))
}