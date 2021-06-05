use serde::{Serialize, Deserialize};
use validator::{Validate};
use actix_web::{web, HttpResponse, FromRequest};
use lily_service::{lilyError, encrypt_text, encrypt_text_bytes};
// use super::{db, db::ReadRow};
// use crate::connection::conn;
use actix_session::Session;
use chrono::{NaiveDateTime, Utc};
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

fn create_user_token(user: &GetUser, session: &Session) -> Result<String, lilyError> {
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

async fn try_login(request: &web::Form<LoginUserInfo>, app_data: &web::Data<App>, session: &Session) -> Result<HttpResponse, lilyError> {
    let conn = app_data.session.get().unwrap();
//   let user = db::read_one_email(&request.email, &con)?;
let mut users = Vec::new();
if let Some(rows) = conn.query(format!("SELECT id, email, password from sankar.userCredentials where email='{}'", &request.email), &[]).await.unwrap().rows {
        for row in rows.into_typed::<GetUser>() {
            let my_row: GetUser = row.unwrap();
            users.push(my_row);
        }
    }
    let user = &users[0];
  let password = encrypt_text(&request.password);
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

pub async fn login(request: web::Form<LoginUserInfo>, app_data: web::Data<App>, session: Session) -> HttpResponse {
  if let Err(error) = request.validate() {
    HttpResponse::Ok().json(error);
  }
  
  match try_login(&request, &app_data, &session).await {
    Ok(res) => res,
    Err(err) => HttpResponse::Ok().json(err)
  }
}