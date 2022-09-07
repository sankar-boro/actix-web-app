use crate::App;
use crate::AppError;

use serde::{Deserialize};
use validator::Validate;
use lily_utils::{time_uuid, encrypt_text};
use scylla::batch::Batch;
use actix_web::{HttpResponse, web};

static INSERT_TABLE__USERS: &str = "INSERT INTO sankar.users (userId,fname,lname, email, password, createdAt, updatedAt) VALUES (?,?,?,?,?,?,?)";
static INSERT_TABLE__USERCREDENTIALS: &str = "INSERT INTO sankar.userCredentials (userId,fname,lname, email, password, createdAt, updatedAt) VALUES (?,?,?,?,?,?,?)";

#[derive(Deserialize, Validate)]
pub struct SignupForm {
    fname: String,
    lname: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

pub async fn signup(session: web::Data<App>, request: web::Json<SignupForm>) -> Result<HttpResponse, actix_web::Error> {
    if let Err(err) = request.validate() {
		return Err(AppError::from(err).into());
	}

    let password = match encrypt_text(&request.password) {
        Ok(pass) => pass,
        Err(err) => return Err(AppError::from(err).into())
    };
    
    let id = time_uuid();
    let mut batch: Batch = Default::default();
    batch.append_statement(INSERT_TABLE__USERS);
    batch.append_statement(INSERT_TABLE__USERCREDENTIALS);

    let fname = &request.fname;
    let lname = &request.lname;
    let email = &request.email;
    let password = password.as_bytes().to_vec();

    let batch_values = (
        (id, fname, &lname, &email, password.clone(),id,id),                
        (id, fname, &lname, &email, password,id,id)
    );

    match session.session.batch(&batch, batch_values).await {
        Ok(_) => Ok(HttpResponse::Ok().body("New user created!")),
        Err(err) => Err(AppError::from(err).into())
    }
}
