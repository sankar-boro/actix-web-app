use actix_session::Session;
use actix_web::{HttpResponse, web};
use scylla::batch::Batch;
use serde::{Deserialize};
use uuid::Uuid;
use crate::App;
use validator::Validate;
use lily_utils::time_uuid;
use scylla::frame::response::cql_to_rust::FromRow;
use scylla::macros::FromRow;
use crate::utils::{
	GetQueryResult, 
	ConnectionResult
};
use crate::AppError;
use crate::auth::auth_session;
use super::payload::{Payload, ParentPayload, PayloadInner, ChildPayload, MakeQuery};


pub async fn create_new_book(
    _app: web::Data<App>, 
    request: web::Json<ParentPayload>,
    session: Session
) 
-> Result<HttpResponse, actix_web::Error> 
{
    
    let conn = _app.conn_result()?;
    let pinner = PayloadInner::PARENT(request.0);    
    let payload = Payload::new(&pinner, &session)?;
    let _: Option<Vec<ParentPayload>> = conn
        .query(payload.query(), 
            &[]
        ).await.get_query_result()?;

    Ok(
        HttpResponse::Ok().json(payload.response())
    )
}

pub async fn create_new_chapter(
    _app: web::Data<App>, 
    request: web::Json<ChildPayload>,
    session: Session
) 
-> Result<HttpResponse, actix_web::Error> 
{
    let conn = _app.conn_result()?;    
    let pinner = PayloadInner::CHILD(request.0);    
    let payload = Payload::new(&pinner, &session)?;
    let _: Option<Vec<ChildPayload>> = conn
        .query(payload.query(), 
            &[]
        ).await.get_query_result()?;

    Ok(
        HttpResponse::Ok().json(payload.response())
    )
}

#[derive(Deserialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct UpdateAndInsert {
    title: String,
    body: String,
    identity: i16,
    bookId: String,
    topUniqueId: String,
    botUniqueId: String,
}
static UP: &str = "UPDATE sankar.book SET parentId=? WHERE bookId=? AND uniqueId=?";
static IT: &str = "INSERT INTO sankar.book (bookId, uniqueId, parentId, authorId, authorName, title, body, identity, createdAt, updatedAt) VALUES (?,?,?,?,?,?,?,?,?,?)";

pub async fn create_and_update_chapter(
    _app: web::Data<App>, 
    payload: web::Json<UpdateAndInsert>,
    session: Session
) 
-> Result<HttpResponse, actix_web::Error> 
{
    let new_id = time_uuid();
    let conn = _app.conn_result()?;
    let mut batch: Batch = Default::default();
    let auth_user = auth_session(&session)?;
    batch.append_statement(UP);
    batch.append_statement(IT);
    let name = format!("{} {}", &auth_user.fname, &auth_user.lname);
    let bo_ok_id = Uuid::parse_str(&payload.bookId).unwrap();
    let u_id = Uuid::parse_str(&payload.botUniqueId).unwrap();
    let p_id = Uuid::parse_str(&payload.topUniqueId).unwrap();
    let a_id = Uuid::parse_str(&auth_user.userId).unwrap();
    let batch_values = (
        (&new_id, bo_ok_id.clone(), &u_id),                
        (bo_ok_id,&new_id,&p_id,&a_id, &name, &payload.title, &payload.body, &payload.identity,&new_id,&new_id)
    );
    match conn.batch(&batch, batch_values).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Updated and created new chapter.")),
        Err(err) => Err(AppError::from(err).into())
    }
}

pub async fn create_new_page(
    _app: web::Data<App>, 
    request: web::Json<ChildPayload>,
    session: Session
) 
-> Result<HttpResponse, actix_web::Error> 
{
    let conn = _app.conn_result()?;    
    let pinner = PayloadInner::CHILD(request.0);    
    let payload = Payload::new(&pinner, &session)?;
    let _: Option<Vec<ChildPayload>> = conn
        .query(payload.query(), 
            &[]
        ).await.get_query_result()?;

    Ok(
        HttpResponse::Ok().json(payload.response())
    )
}

pub async fn create_new_section(
    _app: web::Data<App>, 
    request: web::Json<ChildPayload>,
    session: Session
) 
-> Result<HttpResponse, actix_web::Error> 
{
   let conn = _app.conn_result()?;    
    let pinner = PayloadInner::CHILD(request.0);    
    let payload = Payload::new(&pinner, &session)?;
    let _: Option<Vec<ChildPayload>> = conn
        .query(payload.query(), &[]).await.get_query_result()?;
    Ok(HttpResponse::Ok().json(payload.response()))
}


// HELP
// When creating query, check for commas also. They might cause issue. Right now its working.