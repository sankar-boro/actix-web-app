use uuid::Uuid;
use serde::Deserialize;
use actix_web::{web, HttpResponse};
use serde::{Serialize};

// both of them is required to implement FromRow
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;

use crate::{App, utils::{ConnectionResult, GetQueryResult, Update}};

#[derive(Deserialize)]
pub struct UpdateUserData {
    fname: String,
}

#[derive(FromRow, Serialize)]
pub struct User {
	id: Uuid,
	email: String,
	password: Vec<u8>,
}

pub async fn update_one(session: web::Data<App>, id: web::Path<String>, request: web::Form<UpdateUserData>) 
-> Result<HttpResponse, actix_web::Error> {
    let conn = session.conn_result()?;
    let query = Update::from("sankar.users")
            .set("fname", &request.fname)
            .where_in("userid", &id)
            .query();
    let _: Option<Vec<User>> = conn
    .query(query, &[])
    .await.get_query_result()?;
    Ok(HttpResponse::Ok().body("User updated"))
}
