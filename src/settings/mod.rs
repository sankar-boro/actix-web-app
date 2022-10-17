use actix_session::Session;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{
    App, 
    query::{CREATE_USER_BOOK_SETTINGS, UPDATE_USER_BOOK_SETTINGS}
};
use validator::Validate;
use scylla::{
    batch::Batch,
    macros::FromRow
};
use crate::auth::AuthSession;
use crate::utils::ParseUuid;

// use jsonwebtoken::{encode, Algorithm, Header, EncodingKey};

#[derive(Deserialize, Validate, FromRow)]
pub struct ParentRequest {
    bookId: String,
    settings: String,
}

pub async fn create(
    app: web::Data<App>,
    request: web::Json<ParentRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    let auth = session.user_info()?;
    let author_id = Uuid::parse_str(&auth.userId)?;
    let book_id = &request.bookId.to_uuid()?;

    let create_data = ( 
        &author_id,
        &book_id,
        &request.settings,
    );
    app.query(CREATE_USER_BOOK_SETTINGS, create_data).await?;
    Ok(HttpResponse::Ok().body("Ok."))
}

pub async fn update(
    app: web::Data<App>,
    request: web::Json<ParentRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    let auth = session.user_info()?;
    let author_id = Uuid::parse_str(&auth.userId)?;
    let book_id = &request.bookId.to_uuid()?;

    let create_data = ( 
        &request.settings,
        &author_id,
        &book_id,
    );
    app.query(UPDATE_USER_BOOK_SETTINGS, create_data).await?;
    Ok(HttpResponse::Ok().body("Ok."))
}
