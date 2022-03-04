use serde::{Serialize, Deserialize };
use actix_session::Session;
use crate::AppError;

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