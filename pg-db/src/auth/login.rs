use crate::{error::Error, AppConnections};
use crate::query::LOGIN;

use deadpool_postgres::Pool;
use futures::TryFutureExt;
use validator::Validate;
// use actix_session::Session;
use redis::aio::Connection;
use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use serde_json::json;
use lily_utils::validate_user_credentials;
use actix_web::cookie;
use time::Duration;
use redis::{AsyncCommands, Commands};
use std::{borrow::BorrowMut, sync::{Arc, Mutex}};

#[derive(Deserialize, Debug, Validate)]
pub struct LoginForm {
    #[validate(email)]
	email: String,
	password: String,
}

#[derive(Serialize, Debug)]
pub struct GetUser {
	userId: i32,
	email: String,
	password: String,
	fname: String,
	lname: String,
}

pub async fn login(
	request: web::Json<LoginForm>, 
	app: web::Data<AppConnections>
) 
-> Result<HttpResponse, Error> 
{
	request.validate()?;
	let client = app.db_pool.get().await?;
    let stmt = client.prepare_cached(LOGIN).await?;
    let rows = client.query(&stmt, &[&request.email]).await?;
	if rows.len() == 0 {
		let unf = json!({
			"status": 500,
			"message": "user not found.".to_string(),
		});
		return Ok(HttpResponse::NotFound().json(unf));
	}
	let user_id: i32 = rows[0].get(0);
	let fname: String = rows[0].get(1);
	let lname: String = rows[0].get(2);
	let password: String = rows[0].get(3);
	let password: Vec<u8> = password.as_bytes().to_vec();

	validate_user_credentials(&request.password, &password)?;
	
	let auth_user_session = json!({
		"userId": user_id,
		"email": &request.email.clone(),
		"fname": fname.clone(),
		"lname": lname.clone(),
	});

	let mut locked_session = app.session.lock().unwrap();

	let auth_user_session = auth_user_session.clone().to_string();
	
	locked_session.hset(&request.email, "AUTH_USER", auth_user_session.clone()).unwrap_or_else(|_| ());
	locked_session.hset(&request.email, "AUTH_ID", user_id).unwrap_or_else(|_| ());

	Ok(HttpResponse::Ok()
        .cookie(cookie::Cookie::build("Authorization", "sankar")
            .http_only(true)
            .max_age(Duration::hours(3))
            .same_site(cookie::SameSite::None)
            .secure(true)
            .finish())
        .json(auth_user_session))
}



