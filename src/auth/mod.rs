use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize };
use actix_session::Session;
use crate::AppError;
use crate::App;
use lily_utils::time_uuid;
use crate::query::{CREATE_USER_SESSION};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct AUTHUSER {
    pub userId: String,
    pub fname: String,
    pub lname: String,
    pub email: String,
}

#[allow(dead_code)]
pub fn auth_session(session: &Session) -> Result<AUTHUSER, crate::AppError>  {
    let auth_user = session.get::<String>("AUTH_USER")?;
    match auth_user {
        Some(auth_user) => Ok(serde_json::from_str(&auth_user)?),
        None => return Err(AppError::from("UN_AUTHENTICATED_USER").into())
    }
}

pub trait AuthSession {
    fn user_info(&self) -> Result<AUTHUSER, crate::AppError>;
}

impl  AuthSession for Session {
    fn user_info(&self) -> Result<AUTHUSER, crate::AppError> {
        let auth_user = self.get::<String>("AUTH_USER")?;
        match auth_user {
            Some(auth_user) => Ok(serde_json::from_str(&auth_user)?),
            None => return Err(AppError::from("UN_AUTHENTICATED_USER").into())
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AuthUserSession {
    pub sessionId: String
}

pub async fn generate_session(
    app: web::Data<App>,
    session: Session
) -> Result<HttpResponse, crate::AppError> {

    let auth = session.user_info()?;
    let auth_id_str = auth.userId;
    let auth_id = Uuid::parse_str(&auth_id_str)?;

    let unique_id = time_uuid();
    let create_data = ( 
        &auth_id, // userId
        &unique_id, // sessionId,
        &unique_id, // createdAt
        &unique_id // updatedAt
    );
    app.query(CREATE_USER_SESSION, create_data).await?;

    Ok(HttpResponse::Ok().json(AuthUserSession {
        sessionId: unique_id.to_string(),
    }))
}