use crate::App;
use crate::AppError;
use scylla::macros::FromRow;

use uuid::Uuid;
use chrono::{Utc};
use jsonwebtoken::encode;
use jsonwebtoken::Header;
use validator::{Validate};
use actix_session::Session;
use crate::utils::SessionClaims;
use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{EncodingKey, Algorithm};
use crate::utils::{
	validate_user_credentials, 
	GetQueryResult, 
	ConnectionResult
};
use serde_json::json;

#[derive(Deserialize, Debug, Validate)]
pub struct LoginForm {
    #[validate(email)]
	email: String,
	password: String,
}

// TODO: check if this needs to be pub
#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct UserInfo {
	pub userId: String,
	pub email: String,
	pub fname: String,
	pub lname: String,
}

#[derive(FromRow, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetUser {
	userId: Uuid,
	email: String,
	password: Vec<u8>,
	fname: String,
	lname: String,
}

#[allow(unused)]
fn create_session_token(user: &GetUser) 
-> Result<String, actix_web::Error> {
	let exp = 
		Utc::now()
		.checked_add_signed(chrono::Duration::seconds(3600))
		.expect("valid timestamp")
		.timestamp();
	let claims = 
		SessionClaims::new(
			user.userId, 
			user.email.clone(),
			user.fname.clone(),
			user.lname.clone(), 
			exp, 
			Utc::now().timestamp()
		);
	let header = Header::new(Algorithm::HS512);
	match encode(
		&header, 
		&claims, 
		&EncodingKey::from_secret("secret".as_ref())
	) {
		Ok(a) => Ok(a),
		Err(err) => Err(AppError::from(err).into())
	}
}

fn get_user_query(email: &str) 
-> String {
	let mut query = String::new();
	query.push_str("SELECT userId, email, password, fname, lname from sankar.userCredentials where email='");
	query.push_str(email);
	query.push_str("'LIMIT 1");
	query
}

pub async fn login(
	request: web::Json<LoginForm>, 
	app: web::Data<App>, 
	session: Session
) 
-> Result<HttpResponse, actix_web::Error> 
{
	if let Err(_) = request.validate() {
		return Err(AppError::from("INVALID_CREDENTIALS").into());
	}
	let conn = app.conn_result()?;
	let rows: Option<Vec<GetUser>> = 
		conn.query(get_user_query(&request.email), &[])
		.await
		.get_query_result()?;
	let auth_user: &GetUser = match &rows {
		Some(users) => {
			match users.first() {
				Some(user) => user,
				None => return Err(AppError::from("USER_NOT_FOUND").into())
			}
		},
		None => return Err(AppError::from("USER_NOT_FOUND").into())
	};
	validate_user_credentials(&request.password, &auth_user.password)?;

	// let token = create_session_token(&user)?;
	
	let auth_user_session = json!({
		"userId": auth_user.userId.to_string(),
		"email": auth_user.email.clone(),
		"fname": auth_user.fname.clone(),
		"lname": auth_user.lname.clone(),
	});
	session.insert("AUTH_USER", auth_user_session.clone().to_string())?;
	session.insert("AUTH_ID", &auth_user.userId)?;
	Ok(HttpResponse::Ok().json(auth_user_session))
}