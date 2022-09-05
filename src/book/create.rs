use actix_session::Session;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{App};
use validator::Validate;
use lily_utils::time_uuid;
use scylla::{
    batch::Batch,
    macros::FromRow
};
use crate::auth::AuthSession;

#[derive(Deserialize, Validate, FromRow)]
pub struct ParentRequest {
    title: String,
    body: String,
    identity: i16,
    metadata: String,
}

#[derive(Serialize, Validate, FromRow)]
pub struct ParentResponse {
    bookId: String,
    uniqueId: String,
    parentId: Option<String>,
    title: String,
    body: String,
    identity: i16,
    authorId: String,
    fname: String,
    lname: String,
    metadata: String,
    createdAt: String,
    updatedAt: String,
}

pub static CREATE_BOOK: &str = "INSERT INTO sankar.book (
    bookId, uniqueId, authorId, fname, lname, title, body, identity, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_BOOK_INFO: &str = "INSERT INTO sankar.bookInfo (
    bookId, authorId, fname, lname, title, body, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?
)";

pub async fn create(
    app: web::Data<App>,
    // search: web::Data<Mutex<IndexHandler>>, 
    request: web::Json<ParentRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    let mut batch: Batch = Default::default();
    batch.append_statement(CREATE_BOOK);
    batch.append_statement(CREATE_BOOK_INFO);
    let identity: i16 = 101;

    let auth = session.user_info()?;
    let auth_id_str = auth.userId;
    let auth_id = Uuid::parse_str(&auth_id_str)?;
    let unique_id = time_uuid();
    let unique_id_str = unique_id.to_string();

    let batch_values = (
        (&unique_id, &unique_id, &auth_id, &auth.fname, &auth.lname, &request.title, &request.body, &identity, &unique_id, &unique_id),
        (&unique_id, &auth_id, &auth.fname, &auth.lname, &request.title, &request.body, &request.metadata, &unique_id, &unique_id)
    );

    app.batch(&batch, &batch_values).await?;

    // let a = &mut search.try_lock().unwrap();
    // a.create_document(&request.title, &request.body);

    Ok(
        HttpResponse::Ok().json(ParentResponse {
            bookId: unique_id_str.clone(),
            uniqueId: unique_id_str.clone(),
            parentId: None,
            title: request.title.clone(),
            body: request.body.clone(),
            identity: request.identity.clone(),
            authorId: auth_id.to_string(),
            fname: auth.fname.clone(),
            lname: auth.lname.clone(),
            metadata: request.metadata.clone(),
            createdAt: unique_id_str.clone(),
            updatedAt: unique_id_str.clone(),
        })
    )
}