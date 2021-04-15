use serde::{Serialize, Deserialize};
use validator::{Validate};
use actix_web::{web, HttpResponse};
use loony_service::{encrypt_text, LoonyError};
use super::{db, db::ReadRow};
use crate::connection::conn;
use actix_session::Session;
use chrono::{NaiveDateTime, Utc};
use {serde_json, serde_json::{Value as JsonValue}};
use jsonwebtoken::encode;
use jsonwebtoken::Header;
use jsonwebtoken::{EncodingKey, Algorithm};
use crate::App;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  id: i32,
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

#[derive(Queryable, Serialize, Debug)]
pub struct UserInfo {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub token: String,
}

fn create_user_token(user: &ReadRow, session: &Session) -> Result<String, LoonyError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(1800))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
      id: user.id,
      email: user.email.clone(),
      exp: expiration,
      iat: Utc::now().timestamp()
    };
    let header = Header::new(Algorithm::HS512);
    let token = encode(&header, &claims, &EncodingKey::from_secret("secret".as_ref()))?;
    session.set(&user.id.to_string(), token.clone())?;
    Ok(token)
}

fn try_login(request: &web::Form<LoginUserInfo>, app_data: &web::Data<App>, session: &Session) -> Result<HttpResponse, LoonyError> {
  let con = conn(&app_data)?;
  let user = db::read_one_email(&request.email, &con)?;
  let password = encrypt_text(&request.password);
  if password == user.get_password() {
    let token = create_user_token(&user, session)?;
    return Ok(HttpResponse::Ok().json(UserInfo {
      id: user.id,
      name: user.name,
      email: user.email,
      created_at: user.created_at,
      updated_at: user.updated_at,
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
  
  match try_login(&request, &app_data, &session) {
    Ok(res) => res,
    Err(err) => HttpResponse::Ok().json(err)
  }
}