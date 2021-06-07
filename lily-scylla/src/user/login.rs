use std::fmt::Write;

use crate::App;
use crate::ScyllaConnectionManager;
use crate::service::Error;

use actix_web::http::header;
use r2d2::PooledConnection;
use scylla::QueryResult;
use scylla::macros::FromRow;

use scylla::transport::errors::QueryError;
use uuid::Uuid;
use chrono::{Utc};
use jsonwebtoken::encode;
use jsonwebtoken::Header;
use validator::{Validate};
use scylla::IntoTypedRows;
use actix_session::Session;
use lily_service::{encrypt_text};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{EncodingKey, Algorithm};
use scylla::frame::response::cql_to_rust::FromRow;
use actix_web::{web, HttpResponse, http::StatusCode};

#[derive(Debug, Serialize, Deserialize)]
struct SessionClaims {
	id: String,
	email: String,
	exp: i64,
	iat: i64,
}

#[derive(Deserialize, Debug, Validate)]
pub struct LoginForm {
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

fn create_session_token(user: &GetUser) -> Result<String, jsonwebtoken::errors::Error> {
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
	encode(&header, &claims, &EncodingKey::from_secret("secret".as_ref()))
}

type Conn = PooledConnection<ScyllaConnectionManager>;
type ConnRes = Result<Conn, actix_web::Error>;
trait ConnOrRes {
	fn conn(&self) -> ConnRes;
}

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        self.status_code()
    }

    fn error_response(&self) -> actix_web::BaseHttpResponse<actix_web::body::Body> {
        let mut resp = actix_web::BaseHttpResponse::new(self.status_code());
        let mut buf = web::BytesMut::new();
		buf.write_str(self.get_message().as_str());
        let _ = write!(&mut buf, "{}", self);
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        resp.set_body(actix_web::body::Body::from(buf))
    }
}

impl ConnOrRes for web::Data<App> {

	fn conn(&self) -> ConnRes {
		self.as_ref()
		.conn()
		.map_err(|err| {
			Error::from(err).into()
		})
	}
}

// TODO: 
// login is only working for x-www-form-url-encoded
pub async fn login(request: web::Form<LoginForm>, _app: web::Data<App>, session: Session) -> Result<HttpResponse, actix_web::Error> {

	let a = _app.conn()?;
	todo!();
	// let conn = match _app.conn() {
    //     Ok(conn) => conn,
    //     Err(err) => return err,
    // };
	
	// // Query
	// let mut query = String::new();
	// query.push_str("SELECT id, email, password from sankar.userCredentials where email='");
	// query.push_str(&request.email);
	// query.push_str("'LIMIT 1");
	// // 

	// let users = match conn.query(query, &[]).await {
	// 	Ok(rows) => rows,
	// 	Err(err) =>	{
	// 		return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
	// 		.json(Error::from(err))
	// 	}
	// };

	// // TODO: should recover from unwrap()
    // let users = match users.rows {
    //     Some(users) => {
	// 		users.into_typed::<GetUser>()
	// 		.map(|a| a.unwrap())
	// 		.collect::<Vec<GetUser>>()
	// 	},
    //     None => {
    //         return HttpResponse::build(StatusCode::NOT_FOUND)
	// 		.json(Error::from(format!("User not found with email {}", &request.email)));
    //     },
    // };

	// if users.len() == 0 {
    //     return HttpResponse::build(StatusCode::NOT_FOUND)
	// 	.json(Error::from(format!("User not found with email {}", &request.email)));
	// }

	// let password = match encrypt_text(&request.password) {
	// 	Ok(p) => p,
	// 	Err(err) => {
	// 		return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
	// 		.json(Error::from(err))
	// 	}
	// };

	// let user = &users[0];
	
	// if password.as_bytes() != user.password {
	// 	return HttpResponse::build(StatusCode::BAD_REQUEST)
	// 	.json(Error::from("Invalid credentials".to_string()));
	// } 
	
	// let token = match create_session_token(&user) {
	// 	Ok(token) => token,
	// 	Err(err) => {
	// 		return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
	// 		.json(Error::from(err))
	// 	}
	// };

	// match session.insert(user.id.to_string(), &token) {
	// 	Ok(_) => HttpResponse::Ok().json(UserInfo {
	// 		id: user.id.to_string(),
	// 		email: user.email.clone(),
	// 		token,
	// 	}),
	// 	Err(err) =>  {
	// 		return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
	// 		.json(Error::from(err))
	// 	}
	// }
}