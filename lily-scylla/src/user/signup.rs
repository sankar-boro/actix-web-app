use actix_web::{web, HttpResponse};
use rand::Rng;
use serde::{Deserialize};
use uuid::{Uuid, v1::{Timestamp, Context}};
use crate::App;
use validator::Validate;
use lily_service::encrypt_text;
use chrono::prelude::*;


#[derive(Deserialize, Validate)]
pub struct SignupFormData {
    fname: String,
    lname: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

fn uuid_new() -> Uuid { 
    let mut rng = rand::thread_rng();
    let rand: [u8; 6] = rng.gen();
    let rand_num: u16 = rng.gen();
    let context = Context::new(rand_num);
    let utc: DateTime<Utc> = Utc::now(); 
    let ts = Timestamp::from_unix(&context, utc.timestamp() as u64, utc.timestamp_subsec_nanos());
    Uuid::new_v1(ts, &rand).expect("failed to generate UUID")
}

pub async fn create_user(session: web::Data<App>, request: web::Form<SignupFormData>) -> HttpResponse {
    let conn = session.session.get().unwrap();
    let password = encrypt_text(&request.password);
    let id = uuid_new();
    
    conn
    .query(
        "INSERT INTO sankar.users (
            userId,
            fname,
            lname, 
            email,
            password, 
            createdAt, 
            updatedAt
        ) VALUES(?,?,?,?,?,?,?)", 
        (
            id, 
            &request.fname, 
            &request.lname, 
            &request.email, 
            password.as_bytes().to_vec(),
            id,
            id,
        )
    )
    .await.unwrap();

    conn
    .query(
        "INSERT INTO sankar.userCredentials (
            id, 
            email,
            password
        ) VALUES(?,?,?)", 
        (
            id, 
            &request.email, 
            password.as_bytes().to_vec()
        )
    )
    .await.unwrap();
    HttpResponse::Ok().body("New user created!")
}
