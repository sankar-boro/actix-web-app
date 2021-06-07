use serde::{Serialize, Deserialize};
use validator::{Validate};
use actix_web::{web, HttpResponse};
use lily_service::{encrypt_text};
use actix_session::Session;
use chrono::{Utc};
use jsonwebtoken::encode;
use jsonwebtoken::Header;
use jsonwebtoken::{EncodingKey, Algorithm};
use crate::App;
use scylla::IntoTypedRows;
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;
use uuid::Uuid;
use crate::RequestError;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
	id: String,
	email: String,
	exp: i64,
	iat: i64,
}
#[derive(Deserialize, Debug, Validate)]
pub struct LoginUserInfo {
	#[validate(email)]
	email: String,
	password: String,
}

#[derive(Serialize, Debug)]
pub struct UserInfo {
	pub id: String,
	pub email: String,
	pub token: String,
}

#[derive(FromRow, Serialize)]
struct GetUser {
	id: Uuid,
	email: String,
	password: Vec<u8>,
}

fn res_err(err: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(RequestError::db_error(&err.to_string()))
}

fn bad_req(err: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(RequestError::bad_request(&err.to_string()))
}

fn create_session_token(user: &GetUser) -> Result<String, jsonwebtoken::errors::Error> {
	let expiration = Utc::now()
				.checked_add_signed(chrono::Duration::seconds(1800))
				.expect("valid timestamp")
				.timestamp();

	let claims = Claims {
		id: user.id.to_string(),
		email: user.email.clone(),
		exp: expiration,
		iat: Utc::now().timestamp()
	};

	let header = Header::new(Algorithm::HS512);
	encode(&header, &claims, &EncodingKey::from_secret("secret".as_ref()))
}


// TODO: 
// login is only working for x-www-form-url-encoded
pub async fn login(request: web::Form<LoginUserInfo>, _app: web::Data<App>, session: Session) -> HttpResponse {
	if let Err(error) = request.validate() {
		HttpResponse::BadRequest().json(error);
	}

	let conn = match _app.as_ref().conn() {
        Ok(conn) => conn,
        Err(err) => return res_err(&err.to_string()),
    };
	
	// Query
	let mut query = String::new();
	query.push_str("SELECT id, email, password from sankar.userCredentials where email=");
	let email = format!("'{}'", &request.email);
	query.push_str(&email);
	query.push_str("LIMIT 1");
	// 

	let users = match conn.query(query, &[]).await {
		Ok(rows) => rows,
		Err(err) => return res_err(&err.to_string())
	};

	// TODO: should recover from unwrap()
    let users = match users.rows {
        Some(users) => users.into_typed::<GetUser>().map(|a| a.unwrap()).collect::<Vec<GetUser>>(),
        None => {
            return res_err(&format!("User with email {} not found.", &request.email));
        },
    };

	let password = encrypt_text(&request.password);
	let user = &users[0];
	if password.as_bytes() != user.password {
		return bad_req("Bad credentials.");
	} 
	
	let token = match create_session_token(&user) {
		Ok(token) => token,
		Err(err) => return res_err(&err.to_string())
	};

	match session.insert(user.id.to_string(), &token) {
		Ok(_) => HttpResponse::Ok().json(UserInfo {
			id: user.id.to_string(),
			email: user.email.clone(),
			token,
		}),
		Err(err) =>  res_err(&err.to_string())
	}
}