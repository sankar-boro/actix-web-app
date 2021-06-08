use serde::Deserialize;
use actix_web::{web, HttpResponse};
use crate::{App, AppError, utils::{ConnectionResult, GetQueryResult, Update}};

#[derive(Deserialize)]
pub struct UpdateUserData {
    fname: String,
}

pub async fn update_one(session: web::Data<App>, id: web::Path<String>, request: web::Form<UpdateUserData>) 
-> Result<HttpResponse, actix_web::Error> {
    let conn = session.conn_result()?;
    let query = Update::from("sankar.users")
            .set("fname", &request.fname)
            .where_in("userid", &id)
            .query();
    let res = conn
    .query(query, &[])
    .await;
    match res {
        Ok(_) => {
            Ok(HttpResponse::Ok().body("User updated"))
        }
        Err(err) => {
            return Err(AppError::from(err).into());
        }
    }
}
