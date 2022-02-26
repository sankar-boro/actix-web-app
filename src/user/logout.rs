use actix_web::{HttpResponse};
use actix_session::Session;

pub async fn logout_user(session: Session) -> HttpResponse {
  session.clear();
  session.purge();
  HttpResponse::Ok().body("Logged out.")
}