use actix_session::Session;
use actix_web::{HttpResponse, web};
use serde::{Serialize};
// use uuid::Uuid;
use crate::{Connections, utils::GetQueryResult};
use scylla::{
    macros::FromRow,
    query::Query
};
use crate::auth::AuthSession;

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
    app: web::Data<Connections>
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
    app: web::Data<Connections>,
    session: Session
) 
-> Result<HttpResponse, crate::AppError> 
{
    let auth = session.user_info()?;

    // let query = Query::new(GET_USER_CATEGORIES).with_page_size(4);
    let documents = app.query(GET_USER_CATEGORIES, (auth.userId, ))
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
