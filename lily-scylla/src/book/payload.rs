use serde::{Serialize, Deserialize};
use validator::Validate;
use lily_utils::time_uuid;
use scylla::frame::response::cql_to_rust::FromRow;
use actix_session::Session;
use scylla::macros::FromRow;
use crate::auth::auth_session;

#[allow(non_snake_case)]
pub struct Payload<'a, P>{
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

pub trait MakeQuery {
    type Response;
    fn query(&self) -> String;
    fn response(&self) -> Self::Response;
}

#[derive(Serialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct NewBookResponse {
    pub bookId: String,
    pub uniqueId: String,
    pub parentId: Option<String>,
    pub title: String,
    pub body: String,
    pub identity: i16,
    pub authorId: String,
    pub authorName: String,
    pub createdAt: String,
    pub updatedAt: String,
}

impl<'a> Payload<'a, PayloadInner> {
    pub fn new(p: &'a PayloadInner, session: &Session) -> Result<Self, actix_web::Error> {
        let auth = auth_session(session)?;
        Ok(Self {
            payload: p,
            uuid: time_uuid().to_string(),
            authId: auth.userId,
            authName: format!("{} {}", auth.fname, auth.lname) 
        })
    }
}

struct Res {
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