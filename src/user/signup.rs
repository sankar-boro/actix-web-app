use crate::App;
use crate::AppError;

use serde::{Deserialize};
use validator::{Validate};
use lily_utils::{time_uuid, encrypt_text};
use scylla::batch::Batch;
use actix_web::{HttpResponse, web};
use regex::Regex;
use serde_json::json;
use actix_session::Session;

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
    request: web::Json<SignupForm>,
	session: Session
) -> Result<HttpResponse, crate::AppError> {
    if let Err(err) = request.validate() {
		return Err(AppError::from(err).into());
	}

    let password = match encrypt_text(&request.password) {
        Ok(pass) => pass,
        Err(err) => return Err(AppError::from(err).into())
    };
    
    // let id = time_uuid();
    // let mut batch: Batch = Default::default();
    // batch.append_statement(INSERT_TABLE__USERS);
    // batch.append_statement(INSERT_TABLE__USERCREDENTIALS);

    let fname = &request.fname;
    let lname = &request.lname;
    let email = &request.email.trim();
    let mut client = app.pool.get().await.unwrap();;
    let stmt = client.prepare_cached(INSERT_USER).await.unwrap();
    let rows = client.query(&stmt, &[fname, lname, email, &password]).await.unwrap();
    // let batch_values = (
    //     (id, fname, &lname, &email, password.clone(),id,id),                
    //     (id, fname, &lname, &email, password,id,id)
    // );
    // app.batch(&batch, batch_values).await?;

    let auth_user_session = json!({
        "email": email.clone(),
        "fname": fname.clone(),
        "lname": lname.clone(),
    });
    // session.insert("AUTH_USER", auth_user_session.clone().to_string())?;
    // session.insert("AUTH_ID", id.to_string())?;
    Ok(HttpResponse::Ok().json(auth_user_session))
}
