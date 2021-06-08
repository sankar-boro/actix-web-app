use crate::App;
use crate::AppError;
use crate::ScyllaConnectionManager;

use r2d2::PooledConnection;
use scylla::QueryResult;
use scylla::macros::FromRow;

use uuid::Uuid;
use chrono::{Utc};
use jsonwebtoken::encode;
use jsonwebtoken::Header;
use validator::{Validate};
use scylla::IntoTypedRows;
use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use scylla::transport::errors::QueryError;
use jsonwebtoken::{EncodingKey, Algorithm};
use scylla::frame::response::cql_to_rust::FromRow;
use crate::utils::{validate_password, GetQueryResult, ConnectionResult};

#[derive(Debug, Serialize, Deserialize)]
struct SessionClaims {
	id: String,
	email: String,
	exp: i64,
	iat: i64,
}

#[derive(Deserialize, Debug, Validate)]
pub struct LoginForm {
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
pub struct GetUser {
	id: Uuid,
	email: String,
	password: Vec<u8>,
}

fn create_session_token(user: &GetUser) -> Result<String, actix_web::Error> {
	let expiration = Utc::now()
				.checked_add_signed(chrono::Duration::seconds(1800))
				.expect("valid timestamp")
				.timestamp();

	let claims = SessionClaims {
		id: user.id.to_string(),
		email: user.email.clone(),
		exp: expiration,
		iat: Utc::now().timestamp()
	};

	let header = Header::new(Algorithm::HS512);
	match encode(&header, &claims, &EncodingKey::from_secret("secret".as_ref())) {
		Ok(a) => Ok(a),
		Err(err) => Err(AppError::from(err).into())
	}
}

impl ConnectionResult for web::Data<App> {

	fn conn_result(&self) -> Result<PooledConnection<ScyllaConnectionManager>, actix_web::Error> {
		self.as_ref()
		.conn()
		.map_err(|err| {
			AppError::from(err).into()
		})
	}
}

impl GetQueryResult for Result<QueryResult, QueryError> {
    type Request = GetUser;
	fn get_query_result(self) -> Result<Option<Vec<Self::Request>>, actix_web::Error> {
		self
		.map_err(|err| AppError::from(err).into())
		.map(|res| {
			res.rows.map(|rows| {
				rows.into_typed::<Self::Request>()
					.map(|a| a.unwrap())
					.collect::<Vec<Self::Request>>()
			})
		})
    }
}

// TODO: 
// login is only working for x-www-form-url-encoded
pub async fn login(request: web::Form<LoginForm>, app: web::Data<App>, session: Session) -> Result<HttpResponse, actix_web::Error> {
	if let Err(_) = request.validate() {
		return Err(AppError::from("Invalid credentials.").into());
	}
	let conn = app.conn_result()?;
	
	let mut query = String::new();
	query.push_str("SELECT id, email, password from sankar.userCredentials where email='");
	query.push_str(&request.email);
	query.push_str("'LIMIT 1");
	
	let rows = 
		conn.query(query, &[])
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
	
	validate_password(&request.password, &user.password)?;

	let token = create_session_token(&user)?;

	match session.insert(user.id.to_string(), &token) {
		Ok(_) => Ok(HttpResponse::Ok().json(UserInfo {
			id: user.id.to_string(),
			email: user.email.clone(),
			token,
		})),
		Err(err) => Err(AppError::from(err).into())
	}
}