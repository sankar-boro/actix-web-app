use actix_session::Session;
use uuid::Uuid;
use serde::Deserialize;
use actix_web::{web, HttpResponse};
use serde::{Serialize};
use scylla::macros::FromRow;
use super::queries::UPDATE_USER;

use crate::{App, auth::AuthSession, utils::{ConnectionResult, GetQueryResult}};

#[derive(Deserialize)]
pub struct Request {
    fname: String,
    lname: String,
}

#[derive(FromRow, Serialize)]
pub struct User {
	id: Uuid,
	email: String,
	password: Vec<u8>,
}

pub async fn update_one(app: web::Data<App>, request: web::Json<Request>, session: Session) 
-> Result<HttpResponse, actix_web::Error> {
    
    let conn = app.conn_result()?;
    let auth = session.user_info()?;
    let auth_id = Uuid::parse_str(&auth.userId).unwrap();
    let _: Option<Vec<User>> = conn
    .query(UPDATE_USER, (
        &request.fname, &request.lname, &auth_id
    ))
    .await.get_query_result()?;
    Ok(HttpResponse::Ok().body("User updated"))
}
