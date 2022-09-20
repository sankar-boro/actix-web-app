use actix_session::Session;
use actix_web::{HttpResponse, web};
use lily_utils::time_uuid;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{App, query::{CREATE_USER_CATEGORIES}, utils::GetQueryResult};
use validator::Validate;
use scylla::{
    batch::Batch,
    macros::FromRow,
    query::Query
};
use crate::auth::AuthSession;
// use jsonwebtoken::{encode, Algorithm, Header, EncodingKey};

#[derive(Deserialize, Validate, FromRow)]
pub struct UserCategoryRequest {
    category: Vec<String>,
}

pub async fn create_categories(
    app: web::Data<App>,
    request: web::Json<UserCategoryRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    let mut batch: Batch = Default::default();
    batch.append_statement(CREATE_USER_CATEGORIES);
    let auth = session.user_info()?;
    let auth_id = Uuid::parse_str(&auth.userId)?;
    let unique_id = time_uuid();
    let categories = serde_json::to_string(&request.category)?;
    let batch_values = (
        (&auth_id, &auth_id, &categories, &unique_id, &unique_id),
    );
    app.batch(&batch, &batch_values).await?;
    Ok(
        HttpResponse::Ok().body("Ok")
    )
}

#[derive(FromRow, Serialize)]
pub struct Category {
    category: Uuid,
}

#[derive(Serialize)]
pub struct CategoryResponse {
    categories: Vec<Category>,
    page: Option<Vec<u8>>,
}

static GET_CATEGORIES: &'static str = "SELECT DISTINCT category from sankar.categorybooks";
pub async fn get_categories(
    app: web::Data<App>
) 
-> Result<HttpResponse, crate::AppError> 
{
    let query = Query::new(GET_CATEGORIES).with_page_size(4);
    let documents = app.query(query, &[])
    .await?;
    let page = documents.paging_state.clone();
    let documents: Option<Vec<Category>> = documents.get_query_result()?;
    match documents {
        Some(docs) => {
            let page = match page {
                Some(page) => Some(page.to_vec()),
                None => None,
            };
            Ok(HttpResponse::Ok().json(CategoryResponse{
                categories: docs, 
                page }
            ))
        },
        None => {
            let x: Vec<Category> = Vec::new();
            let y = CategoryResponse{
                categories: x, 
                page: None 
            };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}


static GET_USER_CATEGORIES: &'static str = "SELECT category FROM sankar.usercategories WHERE authorId=?";
pub async fn get_user_categories(
    app: web::Data<App>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    let auth = session.user_info()?;
    let auth_id = Uuid::parse_str(&auth.userId)?;

    // let query = Query::new(GET_USER_CATEGORIES).with_page_size(4);
    let documents = app.query(GET_USER_CATEGORIES, (&auth_id, ))
    .await?;
    let page = documents.paging_state.clone();
    let documents: Option<Vec<Category>> = documents.get_query_result()?;
    match documents {
        Some(docs) => {
            let page = match page {
                Some(page) => Some(page.to_vec()),
                None => None,
            };
            Ok(HttpResponse::Ok().json(CategoryResponse{
                categories: docs, 
                page }
            ))
        },
        None => {
            let x: Vec<Category> = Vec::new();
            let y = CategoryResponse{
                categories: x, 
                page: None 
            };
            Ok(HttpResponse::Ok().json(y))
        },
    }
}
