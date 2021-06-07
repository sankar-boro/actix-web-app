use actix_web::{web, HttpResponse};
use serde::{Deserialize};
use crate::App;
use validator::Validate;
use lily_service::encrypt_text;
use lily_utils::time_uuid;


static INSERT_USER_AT__USERS: &str = "INSERT INTO sankar.users (userId,fname,lname, email, password, createdAt, updatedAt) VALUES (?,?,?,?,?,?,?)";
static INSERT_USER_AT__USER_CREDENTIALS: &str = "INSERT INTO sankar.userCredentials (id, email,password) VALUES(?,?,?)";

#[derive(Deserialize, Validate)]
pub struct SignupFormData {
    fname: String,
    lname: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

pub async fn create_user(session: web::Data<App>, request: web::Form<SignupFormData>) -> HttpResponse {
    let conn = session.session.get().unwrap();
    let password = encrypt_text(&request.password);
    let id = time_uuid();
    
    conn
    .query(
        INSERT_USER_AT__USERS, 
        (id, &request.fname, &request.lname, &request.email, password.as_bytes().to_vec(),id,id)
    )
    .await.unwrap();

    conn
    .query(
        INSERT_USER_AT__USER_CREDENTIALS, 
        (id, &request.email, password.as_bytes().to_vec())
    )
    .await.unwrap();
    HttpResponse::Ok().body("New user created!")
}
