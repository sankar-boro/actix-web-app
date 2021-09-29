use serde::{Serialize, Deserialize };
use actix_session::Session;
use crate::AppError;

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AUTHUSER {
    pub userId: String,
    pub fname: String,
    pub lname: String,
    pub email: String,
}

pub fn auth_session(session: &Session) -> Result<AUTHUSER, actix_web::Error>  {
    let auth_user = session.get::<String>("AUTH_USER")?;
    // let auth_user: AUTHUSER = auth_user.into();
    match auth_user {
        Some(auth_user) => Ok(serde_json::from_str(&auth_user).unwrap()),
        None => return Err(AppError::from("UN_AUTHENTICATED_USER").into())
    }
}
