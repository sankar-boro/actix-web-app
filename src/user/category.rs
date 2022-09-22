use actix_session::Session;
use actix_web::{HttpResponse, web};
use lily_utils::time_uuid;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{App, query::{ADD_CATEGORY,DELETE_CATEGORY}, utils::GetQueryResult};
use validator::Validate;
use scylla::{
    macros::FromRow,
    query::Query
};
use crate::auth::AuthSession;

#[derive(Deserialize, Validate, FromRow)]
pub struct UserCategoryRequest {
    category: String,
}

pub async fn add_category(
    app: web::Data<App>,
    request: web::Json<UserCategoryRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{

    let auth = session.user_info()?;
    let auth_id = Uuid::parse_str(&auth.userId)?;
    let unique_id = time_uuid();
    let _ = app
    .query(ADD_CATEGORY, (&auth_id, &request.category, &unique_id, &unique_id))
    .await?;
    Ok(
        HttpResponse::Ok().body("Ok")
    )
}

pub async fn delete_category(
    app: web::Data<App>,
    request: web::Json<UserCategoryRequest>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{

    let auth = session.user_info()?;
    let auth_id = Uuid::parse_str(&auth.userId)?;
    let _ = app
    .query(DELETE_CATEGORY, (&auth_id, &request.category))
    .await?;
    Ok(
        HttpResponse::Ok().body("Ok")
    )
}

#[derive(FromRow, Serialize)]
pub struct Category {
    category: String,
}

#[derive(Serialize)]
pub struct CategoryResponse {
    categories: Vec<Category>,
    page: Option<Vec<u8>>,
}

static GET_CATEGORIES: &'static str = "SELECT category from sankar.allcategories";
pub async fn get_all_category(
    app: web::Data<App>
) 
-> Result<HttpResponse, crate::AppError> 
{
    let query = Query::new(GET_CATEGORIES).with_page_size(50);
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
