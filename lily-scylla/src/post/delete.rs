use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::App;
use crate::utils::ConnectionResult;

pub async fn delete_one(session: web::Data<App>, id: web::Path<String>)
-> Result<HttpResponse, actix_web::Error> 
{
    let conn = session.conn_result()?;
    let doc_id =  Uuid::parse_str(&id).unwrap();
    conn
    .query("DELETE FROM sankar.documents WHERE documentId=?", (doc_id,))
    .await.unwrap();
    Ok(HttpResponse::Ok().body("Document deleted"))
}
