use actix_web::HttpResponse;
use lily_utils::time_uuid;
use serde::Serialize;
use crate::Error;

#[derive(Serialize)]
pub struct SessionResponse {
    uniqueId: String,
}

pub async fn create_sessionv2() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(SessionResponse{
        uniqueId: time_uuid().to_string()
    }))
}