use actix_session::Session;
use actix_web::{HttpResponse,web};
use crate::Connections;
use uuid::Uuid;
use serde::Serialize;
use crate::error::Error;

use serde_json::json;
use scylla::macros::FromRow;

#[derive(FromRow, Serialize)]
struct GetUser {
    id: Uuid,
    fname: Option<String>,
    lname: Option<String>,
    email: String,
}

static GET_USER: &str = "SELECT fname, lname, email from users where userId=$1";

pub async fn get(app: web::Data<Connections>, path: web::Path<i32>) 
-> Result<HttpResponse, Error> {
    let user_id = path.into_inner();
    let client = app.pool.get().await?;
    let stmt = client.prepare_cached(GET_USER).await?;
    let rows = client.query(&stmt, &[&user_id]).await?;
	if rows.len() == 0 {
		let unf = json!({
			"status": 500,
			"message": "user not found.".to_string(),
		});
		return Ok(HttpResponse::NotFound().json(unf));
	}
	let fname: String = rows[0].get(0);
	let lname: String = rows[0].get(1);
    let email: String = rows[0].get(2);
    
    let auth_user = json!({
		"userId": user_id,
		"email": email,
		"fname": fname,
		"lname": lname,
	});

    Ok(HttpResponse::Ok().json(auth_user))
}

pub async fn user_session(session: Session) 
-> Result<HttpResponse, actix_web::Error> {
    let auth_user_session = session.get::<String>("AUTH_USER")?;
    match auth_user_session {
        Some(session) => {
            Ok(HttpResponse::Ok().body(session))
        }
        None => Err(Error::from("REQUEST_LOGIN").into())   
    }
}