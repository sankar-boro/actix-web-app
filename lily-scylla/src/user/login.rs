use crate::App;
use crate::ScyllaConnectionManager;
use crate::service::Error;

use r2d2::PooledConnection;
use scylla::QueryResult;
use scylla::macros::FromRow;

use uuid::Uuid;
use chrono::{Utc};
use argon2::{Config};
use jsonwebtoken::encode;
use jsonwebtoken::Header;
use validator::{Validate};
use scylla::IntoTypedRows;
use actix_session::Session;
use serde::{Serialize, Deserialize};
use scylla::transport::errors::QueryError;
use jsonwebtoken::{EncodingKey, Algorithm};
use scylla::frame::response::cql_to_rust::FromRow;
use actix_web::{web, HttpResponse, http::StatusCode};
// use crate::service::{validate_password};

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
struct GetUser {
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
		Err(err) => Err(Error::from(err).into())
	}
}

trait ConnectionResult {
	fn conn_result(&self) -> Result<PooledConnection<ScyllaConnectionManager>, actix_web::Error>;
}

impl ConnectionResult for web::Data<App> {

	fn conn_result(&self) -> Result<PooledConnection<ScyllaConnectionManager>, actix_web::Error> {
		self.as_ref()
		.conn()
		.map_err(|err| {
			Error::from(err).into()
		})
	}
}

trait GetQueryResult {
	type Request;
	fn get_query_result(self) -> Result<Option<Vec<Self::Request>>, actix_web::Error>;
}


impl GetQueryResult for Result<QueryResult, QueryError> {
    type Request = GetUser;
	fn get_query_result(self) -> Result<Option<Vec<Self::Request>>, actix_web::Error> {
		self
		.map_err(|err| Error::from(err).into())
		.map(|res| {
			res.rows.map(|rows| {
				rows.into_typed::<Self::Request>()
					.map(|a| a.unwrap())
					.collect::<Vec<Self::Request>>()
			})
		})
    }
}

impl<'a> From<&'a Vec<GetUser>> for &'a GetUser {
    fn from(users: &'a Vec<GetUser>) -> Self {
		&users[0]
    }
}

// TODO: 
// login is only working for x-www-form-url-encoded
pub async fn login(request: web::Form<LoginForm>, app: web::Data<App>, session: Session) -> Result<HttpResponse, actix_web::Error> {
	if let Err(_) = request.validate() {
		return Err(Error::from("Invalid credentials.").into());
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
				None => return Err(Error::from("User not found").into())
			}
		},
		None => return Err(Error::from("User not found").into())
	};
	
	validate_password(&request.password, &user.password)?;

	let token = create_session_token(&user)?;

	match session.insert(user.id.to_string(), &token) {
		Ok(_) => Ok(HttpResponse::Ok().json(UserInfo {
			id: user.id.to_string(),
			email: user.email.clone(),
			token,
		})),
		Err(err) => Err(Error::from(err).into())
	}
}

pub fn validate_password(req_pass: &str, db_pass: &[u8]) -> Result<(), actix_web::Error> {
    let salt = b"sankar_boro";
    let config = Config::default();
    let req_pass = req_pass.as_bytes();
    match argon2::hash_encoded(req_pass, salt, &config) {
      	Ok(data) => {
          	if data.as_bytes() != db_pass {
              	return Err(Error::from("Invalid credentials").into());
          	}
          	Ok(())
      	},
      	Err(err) => Err(Error::from(err).into())
    }
}