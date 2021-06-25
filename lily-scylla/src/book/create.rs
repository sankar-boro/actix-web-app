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
pub struct NewBookPayload {
    title: String,
    body: String,
    identity: i16,
}

#[derive(Deserialize, Validate, FromRow)]
pub struct ChapterPayload {
    bookId: String,
    parentId: String,
    title: String,
    body: String,
    identity: i16,
}

#[derive(Deserialize, Validate, FromRow)]
pub struct SectionPayload {
    bookId: String,
    parentId: String,
    title: String,
    body: String,
    identity: i16,
}

// #[derive(Deserialize, Validate, FromRow)]
// pub struct BookPayload {
//     bookId: String,
//     parentId: String,
//     title: String,
//     body: String,
//     identity: i16,
// }

#[derive(Serialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct NewBookResponse {
    bookId: String,
    uniqueId: String,
    title: String,
    body: String,
    identity: i16,
    authorId: String,
    authorName: String,
    createdAt: String,
    updatedAt: String,
}

#[derive(Serialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct NewChapterResponse {
    bookId: String,
    uniqueId: String,
    parentId: String,
    title: String,
    body: String,
    identity: i16,
    authorId: String,
    authorName: String,
    createdAt: String,
    updatedAt: String,
}

static CREATE_NEW_BOOK: &str = "INSERT INTO sankar.book (
    bookId, uniqueId, authorId, authorName, title, body, identity, createdAt, updatedAt
) VALUES(
    ?,?,?,?,?,?,?,?
)";
static CREATE_NEW_PAGE: &str = "INSERT INTO sankar.book (
    bookId, uniqueId, parentId, authorId, authorName, title, body, identity, createdAt, updatedAt
) VALUES(
    ?,?,?,?,?,?,?,?,?
)";
static CREATE_NEW_CHAPTER: &str = "INSERT INTO sankar.book (
    bookId, uniqueId, parentId, authorId, authorName, title, body, identity, createdAt, updatedAt
) VALUES(
    ?,?,?,?,?,?,?,?,?
)";
static CREATE_NEW_SECTION: &str = "INSERT INTO sankar.book (
    bookId, uniqueId, parentId, authorId, authorName, title, body, identity, createdAt, updatedAt
) VALUES(
    ?,?,?,?,?,?,?,?,?
)";

pub async fn create_new_book(
    _app: web::Data<App>, 
    request: web::Json<NewBookPayload>,
    session: Session
) 
-> Result<HttpResponse, actix_web::Error> 
{
    // get auth user
    let auth_user = auth_session(&session)?; 
    // get conn
    let conn = _app.conn_result()?;    
    // new BOOK id
    let time_uuid = time_uuid();
    let author_name = format!("{} {}", &auth_user.fname, &auth_user.lname);
    // create new BOOK
    let _: Option<Vec<NewBookPayload>> = conn
        .query(CREATE_NEW_BOOK, 
            (
                // primary key and clustering column
                time_uuid,                  // bookId 
                time_uuid,                  // uniqueId
                // parentId not added because this is the parent Id 
                // user Info
                &auth_user.userId,          // authorId
                author_name.clone(),        // authorName 
                // book
                &request.title,             // title
                &request.body,              // body
                &request.identity,          // identity
                // timestamp
                time_uuid,                  // createdAt
                time_uuid                   // updatedAt
            )
        ).await.get_query_result()?;

    // return data of the new BOOK
    let time_uuid = time_uuid.to_string();

    Ok(
        HttpResponse::Ok().json(NewBookResponse { 
            bookId:     time_uuid.clone(), 
            uniqueId:   time_uuid.clone(),
            // user
            authorId:   auth_user.userId,
            authorName: author_name,
            // book
            title:      request.title.to_string(),
            body:       request.body.to_string(),
            identity:   request.identity,
            // timestamp
            createdAt:  time_uuid.clone(),
            updatedAt:  time_uuid.clone(),
        })
    )
}

pub async fn create_new_chapter(
    _app: web::Data<App>, 
    request: web::Json<ChapterPayload>,
    session: Session
) 
-> Result<HttpResponse, actix_web::Error> 
{
    // get auth user
    let auth_user = auth_session(&session)?; 
    // get conn
    let conn = _app.conn_result()?;    
    // new BOOK id
    let time_uuid = time_uuid();
    let author_name = format!("{} {}", &auth_user.fname, &auth_user.lname);
    // create new BOOK
    let _: Option<Vec<NewBookPayload>> = conn
        .query(CREATE_NEW_CHAPTER, 
            (
                // primary key and clustering column
                &request.bookId,            // bookId 
                time_uuid,                  // uniqueId
                &request.parentId,          // parentId
                // user Info
                &auth_user.userId,          // authorId
                author_name.clone(),        // authorName 
                // book
                &request.title,             // title
                &request.body,              // body
                &request.identity,          // identity
                // timestamp
                time_uuid,                  // createdAt
                time_uuid                   // updatedAt
            )
        ).await.get_query_result()?;

    // return data of the new BOOK
    let time_uuid = time_uuid.to_string();

    Ok(
        HttpResponse::Ok().json(NewChapterResponse { 
            bookId:     time_uuid.clone(), 
            uniqueId:   time_uuid.clone(),
            parentId:   request.parentId.clone(),
            // user
            authorId:   auth_user.userId,
            authorName: author_name,
            // book
            title:      request.title.to_string(),
            body:       request.body.to_string(),
            identity:   request.identity,
            // timestamp
            createdAt:  time_uuid.clone(),
            updatedAt:  time_uuid.clone(),
        })
    )
}

pub async fn create_new_page(
    _app: web::Data<App>, 
    request: web::Json<ChapterPayload>,
    session: Session
) 
-> Result<HttpResponse, actix_web::Error> 
{
    // get auth user
    let auth_user = auth_session(&session)?; 
    // get conn
    let conn = _app.conn_result()?;    
    // new BOOK id
    let time_uuid = time_uuid();
    let author_name = format!("{} {}", &auth_user.fname, &auth_user.lname);
    // create new BOOK
    let _: Option<Vec<NewBookPayload>> = conn
        .query(CREATE_NEW_PAGE, 
            (
                // primary key and clustering column
                &request.bookId,            // bookId 
                time_uuid,                  // uniqueId
                &request.parentId,          // parentId
                // user Info
                &auth_user.userId,          // authorId
                author_name.clone(),        // authorName 
                // book
                &request.title,             // title
                &request.body,              // body
                &request.identity,          // identity
                // timestamp
                time_uuid,                  // createdAt
                time_uuid                   // updatedAt
            )
        ).await.get_query_result()?;

    // return data of the new BOOK
    let time_uuid = time_uuid.to_string();

    Ok(
        HttpResponse::Ok().json(NewChapterResponse { 
            bookId:     time_uuid.clone(), 
            uniqueId:   time_uuid.clone(),
            parentId:   request.parentId.clone(),
            // user
            authorId:   auth_user.userId,
            authorName: author_name,
            // book
            title:      request.title.to_string(),
            body:       request.body.to_string(),
            identity:   request.identity,
            // timestamp
            createdAt:  time_uuid.clone(),
            updatedAt:  time_uuid.clone(),
        })
    )
}

pub async fn create_new_section(
    _app: web::Data<App>, 
    request: web::Json<ChapterPayload>,
    session: Session
) 
-> Result<HttpResponse, actix_web::Error> 
{
    // get auth user
    let auth_user = auth_session(&session)?; 
    // get conn
    let conn = _app.conn_result()?;    
    // new BOOK id
    let time_uuid = time_uuid();
    let author_name = format!("{} {}", &auth_user.fname, &auth_user.lname);
    // create new BOOK
    let _: Option<Vec<NewBookPayload>> = conn
        .query(CREATE_NEW_SECTION, 
            (
                // primary key and clustering column
                &request.bookId,            // bookId 
                time_uuid,                  // uniqueId
                &request.parentId,          // parentId
                // user Info
                &auth_user.userId,          // authorId
                author_name.clone(),        // authorName 
                // book
                &request.title,             // title
                &request.body,              // body
                &request.identity,          // identity
                // timestamp
                time_uuid,                  // createdAt
                time_uuid                   // updatedAt
            )
        ).await.get_query_result()?;

    // return data of the new BOOK
    let time_uuid = time_uuid.to_string();

    Ok(
        HttpResponse::Ok().json(NewChapterResponse { 
            bookId:     time_uuid.clone(), 
            uniqueId:   time_uuid.clone(),
            parentId:   request.parentId.clone(),
            // user
            authorId:   auth_user.userId,
            authorName: author_name,
            // book
            title:      request.title.to_string(),
            body:       request.body.to_string(),
            identity:   request.identity,
            // timestamp
            createdAt:  time_uuid.clone(),
            updatedAt:  time_uuid.clone(),
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

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct AUTHUSER {
    userId: String,
    fname: String,
    lname: String,
    email: String,
}

fn auth_session(session: &Session) -> Result<AUTHUSER, actix_web::Error>  {
    let auth_user_id = session.get::<AUTHUSER>("AUTH_USER")?;
    match auth_user_id {
        Some(id) => Ok(id),
        None => return Err(AppError::from("UN_AUTHENTICATED_USER").into())
    }
}


// HELP
// When creating query, check for commas also. They might cause issue. Right now its working.