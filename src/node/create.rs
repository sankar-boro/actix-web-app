use actix_web::{HttpResponse, web};
use scylla::{
    frame::value::ValueList, 
    QueryResult
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::App;
use validator::Validate;
use lily_utils::time_uuid;
use scylla::macros::FromRow;

#[derive(Deserialize, Validate, FromRow)]
pub struct AppendNodeRequest {
    title: String,
    body: String,
    identity: i16,
    bookId: String,
    topUniqueId: String,
}

#[derive(Serialize)]
pub struct Response {
    uniqueId: String,
}

pub static CHILD: &str = "INSERT INTO sankar.book (
    bookId, uniqueId, parentId, title, body, identity, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";

impl AppendNodeRequest {

    async fn query(&self, app: &App, value_list: impl ValueList) -> Result<QueryResult, crate::AppError> {
        Ok(app.query(CHILD, value_list).await?)
    }

    async fn run(&self, app: &App) -> Result<HttpResponse, crate::AppError> {
        // Create and parse elements
        let new_id = time_uuid();
        let book_id = Uuid::parse_str(&self.bookId)?;
        let top_unique_id = Uuid::parse_str(&self.topUniqueId)?;
        let unique_id = new_id.to_string();

        // Create data
        let create_data = ( 
            &book_id,
            &new_id,
            &top_unique_id,
            &self.title,
            &self.body,
            &self.identity,
            &new_id,
            &new_id
        );
        
        self.query(app, create_data).await?;
        
        Ok(HttpResponse::Ok().json(Response {
            uniqueId: unique_id
        }))
    }
}

pub async fn create(
    app: web::Data<App>, 
    payload: web::Json<AppendNodeRequest>
) 
-> Result<HttpResponse, crate::AppError> 
{   
    payload.run(&app).await
}
