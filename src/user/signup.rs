use crate::App;
use crate::AppError;

use regex::Regex;
use serde::{Deserialize};
use validator::{Validate};
use actix_web::{HttpResponse, web};
use lily_utils::{encrypt_text};

lazy_static! {
    static ref MATCH_NAME: Regex = Regex::new(r"^[A-Za-z][A-Za-z0-9_]{2,29}$").unwrap();
}
// static INSERT_TABLE__USERS: &str = "INSERT INTO sankar.users (userId,fname,lname, email, password, createdAt, updatedAt) VALUES (?,?,?,?,?,?,?)";
// static INSERT_TABLE__USERCREDENTIALS: &str = "INSERT INTO sankar.userCredentials (userId,fname,lname, email, password, createdAt, updatedAt) VALUES (?,?,?,?,?,?,?)";
static INSERT_USER: &str = "INSERT INTO users (fname, lname, email, pwd) VALUES ($1, $2, $3, $4)";

#[derive(Deserialize, Validate)]
pub struct SignupForm {
    #[validate(regex = "MATCH_NAME")]
    fname: String,
    #[validate(regex = "MATCH_NAME")]
    lname: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

pub async fn signup(
    app: web::Data<App>, 
    request: web::Json<SignupForm>
) -> Result<HttpResponse, crate::AppError> {
    if let Err(err) = request.validate() {
		return Err(AppError::from(err).into());
	}

    let password = match encrypt_text(&request.password) {
        Ok(pass) => pass,
        Err(err) => return Err(AppError::from(err).into())
    };
    
    let fname = &request.fname;
    let lname = &request.lname;
    let email = &request.email.trim();
    let client = app.pool.get().await?;
    let stmt = client.prepare_cached(INSERT_USER).await?;
    client.query(&stmt, &[fname, lname, email, &password]).await?;
    Ok(HttpResponse::Ok().body("Ok"))
}
