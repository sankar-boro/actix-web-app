use serde::{Serialize, Deserialize};
use validator::{Validate};
use actix_web::{web, HttpResponse};
use lily_service::{WebResponseError, encrypt_text};
use actix_session::Session;
use chrono::{Utc};
use {serde_json, serde_json::{Value as JsonValue}};
use jsonwebtoken::encode;
use jsonwebtoken::Header;
use jsonwebtoken::{EncodingKey, Algorithm};
use crate::App;
use scylla::IntoTypedRows;
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;
use uuid::Uuid;

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

fn create_user_token(user: &GetUser, session: &Session) -> Result<String, WebResponseError> {
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
    let token = encode(&header, &claims, &EncodingKey::from_secret("secret".as_ref()))?;
    session.insert(&user.id.to_string(), token.clone()).unwrap();
    Ok(token)
}

async fn try_login(user: &GetUser, session: &Session, password: &str) -> Result<HttpResponse, WebResponseError> {
    let password = encrypt_text(password);
    if password.as_bytes() == user.password {
        let token = create_user_token(&user, session)?;
        return Ok(HttpResponse::Ok().json(UserInfo {
            id: user.id.to_string(),
          	email: user.email.clone(),
          	token,
        }));
    }

    let data = r#"{"error": "Login failed."}"#;
    let v: JsonValue = serde_json::from_str(data).unwrap();
    Ok(HttpResponse::Ok().json(v))
}


async fn get_user_from_email(web_data: &web::Data<App>, email: &str) -> Result<GetUser, WebResponseError> {
    let conn = web_data.session.get()?;
    let mut query = String::new();
    query.push_str("SELECT id, email, password from sankar.userCredentials where email=");
    let email = format!("'{}'", email);
    query.push_str(&email);
    query.push_str("LIMIT 1");

    if let Some(rows) = conn.query(query, &[]).await.unwrap().rows {
        for row in rows.into_typed::<GetUser>() {
            return Ok(row.unwrap());
        }
    }
    let not_found_error = r#"{"error": "User not found."}"#;
    let v: JsonValue = serde_json::from_str(not_found_error).unwrap();
    Err(WebResponseError::NotFoundError(v))
}


pub async fn login(request: web::Form<LoginUserInfo>, web_data: web::Data<App>, session: Session) -> HttpResponse {
	if let Err(error) = request.validate() {
    	HttpResponse::Ok().json(error);
  	}
	let user = get_user_from_email(&web_data, &request.email).await;
	let user = match user {
		Ok(user) => user,
		Err(err) => {
			return HttpResponse::Ok().json(err);
		}
	};
	let login = try_login(&user, &session, &request.password).await;
	match login {
		Ok(res) => res,
		Err(err) => HttpResponse::Ok().json(err)
	}
}