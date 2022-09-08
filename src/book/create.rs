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
// use jsonwebtoken::{encode, Algorithm, Header, EncodingKey};

#[derive(Deserialize, Validate, FromRow)]
pub struct ParentRequest {
    title: String,
    body: String,
    identity: i16,
    metadata: String,
    uniqueId: String,
    image_url: String,
}

#[derive(Serialize, Validate, FromRow)]
pub struct ParentResponse {
    bookId: String,
    uniqueId: String,
    parentId: Option<String>,
    title: String,
    body: String,
    url: String,
    identity: i16,
    authorId: String,
    fname: String,
    lname: String,
    metadata: String,
    createdAt: String,
    updatedAt: String,
}

pub static CREATE_BOOK: &str = "INSERT INTO sankar.book (
    bookId, uniqueId, authorId, fname, lname, title, body, url, identity, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_BOOK_INFO: &str = "INSERT INTO sankar.bookInfo (
    bookId, authorId, fname, lname, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
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
    let auth_id = Uuid::parse_str(&auth.userId)?;
    let unique_id = Uuid::parse_str(&request.uniqueId)?;
    // let unique_id = time_uuid();
    // let unique_id_str = unique_id.to_string();
    let url = format!("{}/{}", &auth.userId, &request.uniqueId);

    let batch_values = (
        (&unique_id, &unique_id, &auth_id, &auth.fname, &auth.lname, &request.title, &request.body, &request.image_url, &identity, &unique_id, &unique_id),
        (&unique_id, &auth_id, &auth.fname, &auth.lname, &request.title, &request.body, &request.image_url, &request.metadata, &unique_id, &unique_id)
    );

    app.batch(&batch, &batch_values).await?;

    // let a = &mut search.try_lock().unwrap();
    // a.create_document(&request.title, &request.body);

    Ok(
        HttpResponse::Ok().json(ParentResponse {
            bookId: request.uniqueId.clone(),
            uniqueId: request.uniqueId.clone(),
            parentId: None,
            title: request.title.clone(),
            body: request.body.clone(),
            url,
            identity: request.identity.clone(),
            authorId: auth_id.to_string(),
            fname: auth.fname.clone(),
            lname: auth.lname.clone(),
            metadata: request.metadata.clone(),
            createdAt: request.uniqueId.clone(),
            updatedAt: request.uniqueId.clone(),
        })
    )
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Claims {
//    userId: String,
//    contextId: String,
//    exp: usize,
// }

// pub async fn create_book_sessionv1(session:Session) -> Result<HttpResponse, crate::AppError> {
//     let auth = session.user_info()?;
//     let userId = auth.userId;
//     let contextId = time_uuid().to_string();

//     let claims = Claims {
//         userId,
//         contextId,
//         exp: 10000000000
//     };

//     let header =
//     Header { kid: Some("signing_key".to_owned()), alg: Algorithm::HS512, ..Default::default() };

//     let token = encode(&header, &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
//     Ok(HttpResponse::Ok().body(token))
// }

#[derive(Serialize, Validate, FromRow)]
pub struct SessionResponse {
    uniqueId: String,
}
pub async fn create_book_sessionv2() -> Result<HttpResponse, crate::AppError> {
    Ok(HttpResponse::Ok().json(SessionResponse{
        uniqueId: time_uuid().to_string()
    }))
}