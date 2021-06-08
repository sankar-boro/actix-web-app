use crate::App;
use crate::service::Error;

use serde::{Deserialize};
use validator::Validate;
use lily_utils::time_uuid;
use scylla::batch::Batch;
use lily_service::encrypt_text;
use actix_web::{HttpResponse, http::StatusCode, web};

static INSERT_TABLE__USERS: &str = "INSERT INTO sankar.users (userId,fname,lname, email, password, createdAt, updatedAt) VALUES (?,?,?,?,?,?,?)";
static INSERT__TABLE__CREDENTIALS: &str = "INSERT INTO sankar.userCredentials (id, email,password) VALUES(?,?,?)";

#[derive(Deserialize, Validate)]
pub struct SignupForm {
    fname: String,
    lname: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

// TODO: 
// return HttpResponse is too verbal
pub async fn create_user(_app: web::Data<App>, request: web::Form<SignupForm>) -> Result<HttpResponse, actix_web::Error> {
    if let Err(err) = request.validate() {
		return Err(Error::from(err).into());
	}

    let conn = match _app.as_ref().conn() {
        Ok(conn) => conn,
        Err(err) => return Err(Error::from(err).into()),
    };

    let password = match encrypt_text(&request.password) {
        Ok(pass) => pass,
        Err(err) => return Err(Error::from(err).into())
    };
    
    let id = time_uuid();
    let mut batch: Batch = Default::default();
    batch.append_statement(INSERT_TABLE__USERS);
    batch.append_statement(INSERT__TABLE__CREDENTIALS);
    let batch_values = (
        (id, &request.fname, &request.lname, &request.email, password.as_bytes().to_vec(),id,id),                
        (id, &request.email, password.as_bytes().to_vec())
    );

    match conn.batch(&batch, batch_values).await {
        Ok(_) => Ok(HttpResponse::Ok().body("New user created!")),
        Err(err) => Err(Error::from(err).into())
    }
}
