use actix_web::{HttpResponse,web};
use crate::App;
use uuid::Uuid;
use serde::{Serialize};

use scylla::{macros::FromRow, query::Query};
use crate::utils::{
	GetQueryResult
};

#[derive(FromRow, Serialize)]
pub struct BookMetadata {
    bookId: Uuid,
    parentId: Uuid,
    uniqueId: Uuid,
    title: String,
    identity: i16
}

static BOOK_TITLES: &'static str = "SELECT bookId, parentId, uniqueId, title, identity from sankar.book_title";

pub async fn get_titles(
    app: web::Data<App>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let query = Query::new(BOOK_TITLES);
    let query_res = app.query(query, &[]).await?;
    let documents: Option<Vec<BookMetadata>> = query_res.get_query_result()?;
    Ok(HttpResponse::Ok().json(documents))
}