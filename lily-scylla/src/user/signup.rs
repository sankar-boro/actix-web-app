use actix_web::{HttpResponse, http::StatusCode, web};
use serde::{Deserialize};
use crate::App;
use validator::Validate;
use lily_service::encrypt_text;
use lily_utils::time_uuid;
use crate::RequestError;
use scylla::batch::Batch;
use serde::Serialize;
use argon2;


static INSERT_TABLE__USERS: &str = "INSERT INTO sankar.users (userId,fname,lname, email, password, createdAt, updatedAt) VALUES (?,?,?,?,?,?,?)";
static INSERT__TABLE__CREDENTIALS: &str = "INSERT INTO sankar.userCredentials (id, email,password) VALUES(?,?,?)";

#[derive(Deserialize, Validate)]
pub struct SignupFormData {
    fname: String,
    lname: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
}

#[derive(Serialize)]
struct Error {
    status: u16,
    message: String,
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: e.to_string(),
        }
    }
}

impl From<argon2::Error> for Error {
    fn from(e: argon2::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: e.to_string(),
        }
    }
}

impl From<scylla::transport::errors::QueryError> for Error {
    fn from(e: scylla::transport::errors::QueryError) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: e.to_string(),
        }
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(e: validator::ValidationErrors) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: e.to_string(),
        }
    }
}

pub async fn create_user(_app: web::Data<App>, request: web::Form<SignupFormData>) -> HttpResponse {
    if let Err(err) = request.validate() {
		return HttpResponse::build(StatusCode::BAD_REQUEST).json(Error::from(err));
	}

    let conn = match _app.as_ref().conn() {
        Ok(conn) => conn,
        Err(err) => return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(Error::from(err)),
    };

    let password = match encrypt_text(&request.password) {
        Ok(pass) => pass,
        Err(err) => return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(Error::from(err))
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
        Ok(_) => HttpResponse::Ok().body("New user created!"),
        Err(err) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(Error::from(err))
    }
}
