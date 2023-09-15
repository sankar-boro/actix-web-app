use crate::error::Error;

use deadpool_postgres::Pool;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct GetUserX {
	email: String,
}

static GET_USERX: &str = "SELECT userId, fname, lname, pwd FROM users WHERE email=$1";

pub async fn get_user(
	request: web::Json<GetUserX>, 
	app: web::Data<Pool>
) 
-> Result<HttpResponse, Error> 
{
	let client = app.get().await?;
    let stmt = client.prepare_cached(GET_USERX).await?;
    let rows = client.query(&stmt, &[&request.email]).await?;
	let user_id: i32 = rows[0].get(0);
	let fname: String = rows[0].get(1);
	let lname: String = rows[0].get(2);
	
	let auth_user_session = json!({
		"userId": user_id.clone(),
		"email": &request.email.clone(),
		"fname": fname.clone(),
		"lname": lname.clone(),
	});
	Ok(HttpResponse::Ok().json(auth_user_session))
}