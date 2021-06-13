use crate::App;
use crate::AppError;
use crate::session;
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
use scylla::frame::response::cql_to_rust::FromRow;
use crate::utils::{
	validate_user_credentials, 
	GetQueryResult, 
	ConnectionResult
};

#[derive(Deserialize, Debug, Validate)]
pub struct LoginForm {
    #[validate(email)]
	email: String,
	password: String,
}

// TODO: check if this needs to be pub
#[derive(Serialize, Debug)]
pub struct UserInfo {
	pub id: String,
	pub email: String,
	pub fname: String,
	pub lname: String,
	pub token: String,
}

#[derive(FromRow, Serialize, Debug)]
pub struct GetUser {
	id: Uuid,
	email: String,
	password: Vec<u8>,
	fname: String,
	lname: String,
}

fn create_session_token(user: &GetUser) 
-> Result<String, actix_web::Error> {
	let exp = 
		Utc::now()
		.checked_add_signed(chrono::Duration::seconds(3600))
		.expect("valid timestamp")
		.timestamp();
	let claims = 
		SessionClaims::new(
			user.id, 
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

pub async fn login(request: web::Json<LoginForm>, app: web::Data<App>, session: Session) 
-> Result<HttpResponse, actix_web::Error> {
	if let Err(_) = request.validate() {
		return Err(AppError::from("Invalid credentials.").into());
	}
	let conn = app.conn_result()?;
	let rows: Option<Vec<GetUser>> = 
		conn.query(get_user_query(&request.email), &[])
		.await
		.get_query_result()?;
	let user: &GetUser = match &rows {
		Some(users) => {
			match users.first() {
				Some(user) => user,
				None => return Err(AppError::from("User not found").into())
			}
		},
		None => return Err(AppError::from("User not found").into())
	};
	validate_user_credentials(&request.password, &user.password)?;
	let token = create_session_token(&user)?;
	session.insert("session", &token)?;
	Ok(HttpResponse::Ok().json(UserInfo {
		id: user.id.to_string(),
		email: user.email.clone(),
		token,
		fname: user.fname.clone(),
		lname: user.lname.clone(),
	}))
}