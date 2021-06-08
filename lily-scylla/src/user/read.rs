use actix_web::{HttpResponse,web};
use crate::App;
use crate::utils::ConnectionResult;
use uuid::Uuid;
use serde::Serialize;
use crate::AppError;

use scylla::IntoTypedRows;
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;
use crate::utils::{GetQueryResult};

#[derive(FromRow, Serialize)]
struct GetUser {
    id: Uuid,
    fname: Option<String>,
    lname: Option<String>,
    email: Option<String>,
}

static GET_ALL_TABLE_USERS: &str = "SELECT userId, fname, lname, email from sankar.users";
pub async fn get_all(session: web::Data<App>) 
-> Result<HttpResponse, actix_web::Error> {
    let conn = session.conn_result()?;
    let rows: Option<Vec<GetUser>> = 
		conn.query(GET_ALL_TABLE_USERS, &[])
		.await
		.get_query_result()?;
    match rows {
        Some(rows) => {
            Ok(HttpResponse::Ok().json(rows))
        }
        None => {
            let mt: Vec<GetUser> = Vec::new();
            Ok(HttpResponse::Ok().json(mt))
        }
    }
}



// Section //\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
fn get_user_query(user_id: &str) 
-> Result<String, actix_web::Error> {
    match Uuid::parse_str(user_id) {
        Ok(user_id) => {
            Ok(format!("SELECT userId, fname, lname, email from sankar.users where userId={} LIMIT 1", user_id))
        }
        Err(err) => Err(AppError::from(err).into())
    }
}
pub async fn get_one(session: web::Data<App>, user_id: web::Path<String>,) 
-> Result<HttpResponse, actix_web::Error> {
    let conn = session.conn_result()?;
    let rows: Option<Vec<GetUser>> = 
		conn.query(get_user_query(&user_id)?, &[])
		.await
		.get_query_result()?;
    match rows {
        Some(rows) => {
            Ok(HttpResponse::Ok().json(rows))
        }
        None => {
            let mt: Vec<GetUser> = Vec::new();
            Ok(HttpResponse::Ok().json(mt))
        }
    }
}