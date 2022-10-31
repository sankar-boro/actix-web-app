use actix_web::{HttpResponse, web};
use scylla::{
    batch::Batch, 
    frame::value::BatchValues,
    BatchResult
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{App, utils::ParseUuid};
use validator::Validate;
use scylla::macros::FromRow;
use actix_session::Session;
use crate::{auth::AuthSession};

#[derive(Deserialize, Validate, FromRow)]
pub struct MergeNodeRequest {
    uniqueId: String,
    title: String,
    body: String,
    identity: i16,
    blogId: String,
    metadata: String,
    topUniqueId: String,
    botUniqueId: String,
    image_url: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    uniqueId: String,
}

pub static UPDATE_PARENT_ID: &str = "UPDATE sankar.blog SET parentId=? WHERE blogId=? AND uniqueId=?";
pub static CHILD: &str = "INSERT INTO sankar.blog (
    blogId, uniqueId, parentId, authorId, title, body, identity, metadata, url, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)";

impl MergeNodeRequest {

    async fn batch(&self, app: &App, batch_values: impl BatchValues) -> Result<BatchResult, crate::AppError> {
        let mut batch: Batch = Default::default();
        batch.append_statement(UPDATE_PARENT_ID);
        batch.append_statement(CHILD);
        Ok(app.batch(&batch, batch_values).await?)
    }

    async fn run(&self, app: &App, session: &Session) -> Result<HttpResponse, crate::AppError> {
        let auth = session.user_info()?;
        let author_id = Uuid::parse_str(&auth.userId)?;

        // Create and parse elements
        let new_id = &self.uniqueId;
        let new_id = new_id.to_uuid()?;
        let blog_id = Uuid::parse_str(&self.blogId)?;
        let top_unique_id = Uuid::parse_str(&self.topUniqueId)?;
        let bot_unique_id = Uuid::parse_str(&self.botUniqueId)?;

        let mut image_url = None;
        if let Some(b) = &self.image_url {
            image_url = Some(b.to_owned());
        }

        // Create data
        let create_data = ( 
            &blog_id,
            &new_id,
            &top_unique_id,
            &author_id,
            &self.title,
            &self.body,
            &self.identity,
            &self.metadata,
            &image_url,
            &new_id,
            &new_id
        );

        let update_data = (
            &new_id,
            &blog_id,
            bot_unique_id
        );
        let batch_values = (
            update_data,
            create_data
        );
        self.batch(app, batch_values).await?;

        Ok(HttpResponse::Ok().json(Response {
            uniqueId: self.uniqueId.to_owned()
        }))
    }
}

pub async fn merge(
    app: web::Data<App>, 
    payload: web::Json<MergeNodeRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{   
    payload.run(&app, &session).await
}
