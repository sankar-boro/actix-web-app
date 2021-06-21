use actix_session::Session;
use actix_web::dev::ServiceRequest;
use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize};
use uuid::Uuid;
use crate::App;
use validator::Validate;
use lily_utils::time_uuid;
use scylla::frame::response::cql_to_rust::FromRow;
use scylla::macros::FromRow;
use crate::utils::{
	GetQueryResult, 
	ConnectionResult
};
use serde::Serialize;
use crate::AppError;

#[derive(Deserialize, Validate, FromRow)]
pub struct BookPayload {
    title: String,
    description: String,
}

#[derive(Serialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct BookResponse {
    bookId: String,
    title: String,
    description: String,
    authorId: String,
}

static INSERT_BOOK_INTO__BOOKS: &str = "INSERT INTO sankar.books (bookId,title,description,authorId) VALUES(?,?,?,?)";

pub async fn create_one(
    _app: web::Data<App>, 
    request: web::Json<BookPayload>,
    session: Session
) 
-> Result<HttpResponse, actix_web::Error> 
{
    // get auth user
    let auth_user_id = auth_id(&session)?; 

    // get conn
    let conn = _app.conn_result()?;    
    
    // new BOOK id
    let book_id = time_uuid();

    // create new BOOK
    let _: Option<Vec<BookPayload>> = conn
        .query(INSERT_BOOK_INTO__BOOKS, 
            (book_id, &request.title,&request.description, &auth_user_id)
        ).await.get_query_result()?;

    // return data of the new BOOK

    Ok(
        HttpResponse::Ok().json(BookResponse { 
            bookId: book_id.to_string(), 
            title: request.title.to_string(),
            description: request.description.to_string(),
            authorId: auth_user_id.to_string(),
        })
    )
}


fn auth_id(session: &Session) -> Result<Uuid, actix_web::Error>  {
    let auth_user_id = session.get::<String>("AUTH_ID")?;
    let auid = match auth_user_id {
        Some(id) => id,
        None => return Err(AppError::from("UN_AUTHENTICATED_USER").into())
    };

    match Uuid::parse_str(&auid) {
        Ok(aid) => Ok(aid),
        Err(e) => Err(AppError::from(e).into())
    }
}


// HELP
// When creating query, check for commas also. They might cause issue. Right now its working.
