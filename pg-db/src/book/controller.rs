use crate::query::{BOOK_DATA, ALL_BOOKS};
use deadpool_postgres::Pool;
use actix_session::Session;
use actix_web::{HttpResponse, web};
use serde_json::json;
use crate::error::Error;
use super::model::{GetBook, GetBooks};

pub async fn get_all_books(
    app: web::Data<Pool>,
    _: Session
) 
-> Result<HttpResponse, Error> 
{
    let conn = app.get().await?;
    let books = conn.query(
        ALL_BOOKS, 
        &[]
    ).await?;

    let mut allbooks = Vec::new();
    for i in 0..books.len() {
        allbooks.push(GetBooks {
            uid: books[i].get(0),
            authorid: books[i].get(1),
            title: books[i].get(2),
            body: books[i].get(3),
            metadata: books[i].get(4),
            createdat: books[i].get(5)
        });
    }

    Ok(HttpResponse::Ok().json(json!({
        "status": 200,
        "data": allbooks
    })))
}

pub async fn get_all_book_nodes(
    app: web::Data<Pool>,
    path: web::Path<String>,
    _: Session
) 
-> Result<HttpResponse, Error> 
{
	let bookid: i32 = path.parse()?;
    let conn = app.get().await?;
    let books = conn.query(
        BOOK_DATA, 
        &[&bookid]
    ).await?;

    let mut allbooks = Vec::new();
    for i in 0..books.len() {
        allbooks.push(GetBook {
            uid: books[i].get(0),
            authorid: books[i].get(1),
            bookid: books[i].get(2),
            parentid: books[i].get(3),
            title: books[i].get(4),
            body: books[i].get(5),
            identity: books[i].get(6),
            metadata: books[i].get(7),
            createdat: books[i].get(8)
        });

    }

    Ok(HttpResponse::Ok().json(json!({
        "status": 200,
        "data": allbooks
    })))
}