use crate::error::Error;
use crate::query::LOGIN;

use deadpool_postgres::Pool;
use validator::Validate;
use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use serde_json::json;
use lily_utils::validate_user_credentials;

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
	app: web::Data<Pool>, 
	session: Session
) 
-> Result<HttpResponse, Error> 
{
	request.validate()?;
	let client = app.get().await?;
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
	let pwd: String = rows[0].get(3);
	let pwd: Vec<u8> = pwd.as_bytes().to_vec();

	validate_user_credentials(&request.password, &pwd)?;
	
	let auth_user_session = json!({
		"userId": user_id,
		"email": &request.email.clone(),
		"fname": fname.clone(),
		"lname": lname.clone(),
	});
	let x = auth_user_session.clone().to_string();
	
	session.insert("AUTH_USER", x)?;
	session.insert("AUTH_ID", user_id)?;
	Ok(HttpResponse::Ok().json(auth_user_session))
}



