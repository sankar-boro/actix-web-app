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
use serde::Serialize;
use crate::AppError;

#[allow(non_snake_case)]
struct Payload<'a, P>{
    payload:&'a P,
    uuid: String,
    authId: String,
    authName: String,
}
#[derive(Deserialize, Validate, FromRow)]
pub struct ParentPayload {
    title: String,
    body: String,
    identity: i16,
}

#[derive(Deserialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct ChildPayload {
    title: String,
    body: String,
    identity: i16,
    bookId: String,
    parentId: String,
}
pub enum PayloadInner {
    PARENT(ParentPayload),
    CHILD(ChildPayload)
}

trait MakeQuery {
    type Response;
    fn query(&self) -> String;
    fn response(&self) -> Self::Response;
}
impl<'a> Payload<'a, PayloadInner> {
    fn new(p: &'a PayloadInner, session: &Session) -> Result<Self, actix_web::Error> {
        let auth = auth_session(session)?;
        Ok(Self {
            payload: p,
            uuid: time_uuid().to_string(),
            authId: auth.userId,
            authName: format!("{} {}", auth.fname, auth.lname) 
        })
    }
}
impl<'a> MakeQuery for Payload<'a, PayloadInner> {
    type Response = NewBookResponse;
    fn query(&self) -> String {
        match self.payload {
            PayloadInner::PARENT(payload) => {
                return format!(
                "INSERT INTO sankar.book (
	                bookId, uniqueId, authorId, authorName, title, body, identity, createdAt, updatedAt
                ) VALUES(
                    {},{},{},'{}','{}','{}',{},{},{}
                )", 
                self.uuid,self.uuid,self.authId,self.authName,&payload.title, &payload.body, &payload.identity,self.uuid,self.uuid);
            },
            PayloadInner::CHILD(payload) => {
            return format!(
                "INSERT INTO sankar.book (
                    bookId, uniqueId, parentId, authorId, authorName, title, body, identity, createdAt, updatedAt
                ) VALUES(
                    {},{},{},{},'{}','{}','{}',{},{},{}
                )", 
                payload.bookId,self.uuid,payload.parentId,self.authId, self.authName,&payload.title, &payload.body, &payload.identity,self.uuid,self.uuid);
            },
        }
        
    }

    fn response(&self) -> Self::Response {
        match self.payload {
            PayloadInner::PARENT(payload) => {
                return NewBookResponse { 
                    bookId:     self.uuid.clone(), 
                    uniqueId:   self.uuid.clone(),
                    parentId:   None,
                    // user
                    authorId:   self.authId.clone(),
                    authorName: self.authName.clone(),
                    // book
                    title:      payload.title.to_string(),
                    body:       payload.body.to_string(),
                    identity:   payload.identity,
                    // timestamp
                    createdAt:  self.uuid.clone(),
                    updatedAt:  self.uuid.clone(),
                };
            },
            PayloadInner::CHILD(payload) => {
                return NewBookResponse { 
                    bookId:     self.uuid.clone(), 
                    uniqueId:   self.uuid.clone(),
                    parentId:   Some(payload.parentId.clone()),
                    // user
                    authorId:   self.authId.clone(),
                    authorName: self.authName.clone(),
                    // book
                    title:      payload.title.clone(),
                    body:       payload.body.clone(),
                    identity:   payload.identity,
                    // timestamp
                    createdAt:  self.uuid.clone(),
                    updatedAt:  self.uuid.clone(),
                };
            },
        }
    }
}

#[derive(Serialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct NewBookResponse {
    bookId: String,
    uniqueId: String,
    parentId: Option<String>,
    title: String,
    body: String,
    identity: i16,
    authorId: String,
    authorName: String,
    createdAt: String,
    updatedAt: String,
}


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
    println!("{}", &UP);
    println!("{}", &IT);
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
        .query(payload.query(), 
            &[]
        ).await.get_query_result()?;

    Ok(
        HttpResponse::Ok().json(payload.response())
    )
}



fn auth_id(session: &Session) -> Result<Uuid, actix_web::Error>  {
    let auth_user_id = session.get::<String>("AUTH_ID")?;
    let auid = match auth_user_id {
        Some(id) => id,
        None => return Err(AppError::from("UN_AUTHENTICATED_USER").into())
    };

    match Uuid::parse_str(&auid) {
        Ok(aid) => Ok(aid),
        Err(e) => Err(AppError::from(e).into())
    }
}

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
struct AUTHUSER {
    userId: String,
    fname: String,
    lname: String,
    email: String,
}

fn auth_session(session: &Session) -> Result<AUTHUSER, actix_web::Error>  {
    let auth_user = session.get::<String>("AUTH_USER")?;
    // let auth_user: AUTHUSER = auth_user.into();
    match auth_user {
        Some(auth_user) => Ok(serde_json::from_str(&auth_user).unwrap()),
        None => return Err(AppError::from("UN_AUTHENTICATED_USER").into())
    }
}


// HELP
// When creating query, check for commas also. They might cause issue. Right now its working.