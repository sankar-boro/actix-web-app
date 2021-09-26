use uuid::Uuid;
use crate::App;
use serde::Deserialize;
use validator::Validate;
use scylla::macros::FromRow;
use actix_web::{web, HttpResponse};
use crate::utils::{ConnectionResult};
use scylla::frame::response::cql_to_rust::FromRow;
use scylla::batch::Batch;
use crate::AppError;
use scylla::query::Query;


#[derive(Deserialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct Delete {
    bookId: String,
    uniqueId: String,
}

pub async fn delete_section_last(
    session: web::Data<App>, 
    payload: web::Json<Delete>
)
-> Result<HttpResponse, actix_web::Error> 
{
    let conn = session.conn_result()?;
    let book_id =  Uuid::parse_str(&payload.bookId).unwrap();
    let unique_id =  Uuid::parse_str(&payload.uniqueId).unwrap();
    conn
    .query("DELETE FROM sankar.book WHERE bookId=? AND uniqueId=?", (book_id, unique_id))
    .await.unwrap();
    Ok(HttpResponse::Ok().body("Document deleted"))
}

#[derive(Deserialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct PayloadUpdateAndDelete {
    data: String,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct UpdateData {
    bookId: String,
    uniqueId: String,
    newParentId: String,
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct DeleteData {
    bookId: String,
    deleteUniqueId: String,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct UpdateAndDelete {
    updateData: UpdateData,
    deleteData: DeleteData,
}

pub async fn delete_section_first(
    session: web::Data<App>, 
    payload: web::Json<PayloadUpdateAndDelete>
)
-> Result<HttpResponse, actix_web::Error> 
{
    println!("{}", payload.data);

    let conn = session.conn_result()?;
    let p: UpdateAndDelete = serde_json::from_str(&payload.data).unwrap();
    let u = p.updateData;
    let d = p.deleteData;

    let mut batch: Batch = Default::default();
    batch.append_statement("UPDATE sankar.book SET parentId=? WHERE bookId=? AND uniqueId=?");
    batch.append_statement("DELETE FROM sankar.book WHERE bookId=? AND uniqueId=?");

    let book_id = Uuid::parse_str(&u.bookId).unwrap();
    let new_parent_id = Uuid::parse_str(&u.newParentId).unwrap();
    let unique_id = Uuid::parse_str(&u.uniqueId).unwrap();
    let delete_id = Uuid::parse_str(&d.deleteUniqueId).unwrap();
    let batch_values = (
        (&new_parent_id, &book_id, &unique_id),                
        (&book_id, &delete_id)
    );

    match conn.batch(&batch, batch_values).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Updated and created new chapter.")),
        Err(err) => Err(AppError::from(err).into())
    }
}

#[derive(Deserialize)]
struct SectionUpdateData {
    uniqueId: String,
    newParentId: String,
}
#[derive(Deserialize)]
struct SectionDeleteData {
    uniqueId: String,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct DeleteSectionInner{
    updateData: SectionUpdateData,
    deleteData: Vec<SectionDeleteData>
}

#[derive(Deserialize, Validate, FromRow)]
#[allow(non_snake_case)]
pub struct DeleteSection {
    bookId: String,
    data: String,
}

pub async fn delete_main_section(
    session: web::Data<App>, 
    payload: web::Json<DeleteSection>
)
-> Result<HttpResponse, actix_web::Error> 
{
    let conn = session.conn_result()?;
    let p: DeleteSectionInner = serde_json::from_str(&payload.data).unwrap();
    let u = p.updateData;
    let d = p.deleteData;

    let mut batch: Batch = Default::default();
    let book_id = Uuid::parse_str(&payload.bookId).unwrap();

    let g: Query = Query::new(format!("UPDATE sankar.book SET parentId={} WHERE bookId={} AND uniqueId={}", &u.newParentId, &book_id, &u.uniqueId));
    batch.append_statement(g);

    let mut d_query = format!("DELETE FROM sankar.book WHERE bookId={} AND uniqueId IN (", &book_id);
    for (_i, f) in d.iter().enumerate() {
        let u_id = Uuid::parse_str(&f.uniqueId).unwrap();
        if _i == 0 {
            d_query.push_str(&format!("{}", &u_id));
        } else {
            d_query.push_str(&format!(",{}", &u_id));    
        }
    }
    d_query.push_str(")");
    println!("{}", &d_query);
    let q: Query = Query::new(d_query);
    batch.append_statement(q);

    match conn.batch(&batch, ((), ())).await {
        Ok(_) => Ok(HttpResponse::Ok().body("Updated and created new chapter.")),
        Err(err) => Err(AppError::from(err).into())
    }
}
