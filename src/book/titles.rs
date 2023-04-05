use actix_web::{HttpResponse,web};
use crate::Connections;
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

static BOOK_TITLES: &'static str = "SELECT bookId, parentId, uniqueId, title, identity from sankar.book_title WHERE bookId=?";

#[derive(Serialize)]
pub struct TitleResponse {
    nodes: Option<Vec<BookMetadata>>
}

pub async fn get_book_titles(
    app: web::Data<Connections>,
    book_id: web::Path<String>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    let get_book_id = Uuid::parse_str(&book_id)?;

    let query = Query::new(BOOK_TITLES);
    let query_res = app.query(query, (&get_book_id,)).await?;
    let nodes: Option<Vec<BookMetadata>> = query_res.get_query_result()?;
    Ok(HttpResponse::Ok().json(TitleResponse { nodes}))
}