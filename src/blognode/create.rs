use actix_web::{HttpResponse, web};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::App;
use validator::Validate;
use lily_utils::time_uuid;
use scylla::macros::FromRow;
use crate::query::{CREATE_BLOG_NODE_QUERY};

#[derive(Deserialize, Validate, FromRow)]
pub struct AppendNodeRequest {
    title: String,
    body: String,
    identity: i16,
    blogId: String,
    topUniqueId: String,
}

#[derive(Serialize)]
pub struct Response {
    uniqueId: String,
}

trait ParseUuid {
    fn to_uuid(self) -> Result<Uuid, crate::AppError>;
}

impl ParseUuid for &String {
    fn to_uuid(self) -> Result<Uuid, crate::AppError> {
        Ok(Uuid::parse_str(self)?)
    }
}

impl AppendNodeRequest {
    
    async fn run(&self, app: &App) -> Result<HttpResponse, crate::AppError> {
        let new_id = time_uuid();
        let blog_id = self.blogId.to_uuid()?;
        let top_unique_id = self.topUniqueId.to_uuid()?;
        let unique_id = new_id.to_string();
        let create_data = ( 
            &blog_id,
            &new_id,
            &top_unique_id,
            &self.title,
            &self.body,
            &self.identity,
            &new_id,
            &new_id
        );
        app.query(CREATE_BLOG_NODE_QUERY, create_data).await?;
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
