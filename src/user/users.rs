use actix_web::{HttpResponse,web};
use crate::Connections;
use uuid::Uuid;
use serde::Serialize;

use scylla::macros::FromRow;
use crate::utils::{GetQueryResult};

#[derive(FromRow, Serialize)]
struct GetUser {
    id: Uuid,
    fname: Option<String>,
    lname: Option<String>,
    email: Option<String>,
}

static GET_ALL_TABLE_USERS: &str = "SELECT userId, fname, lname, email from sankar.users";
pub async fn users(app: web::Data<Connections>) 
-> Result<HttpResponse, crate::AppError> {
    let rows: Option<Vec<GetUser>> = 
		app.query(GET_ALL_TABLE_USERS, &[])
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