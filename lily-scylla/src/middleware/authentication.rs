use std::pin::Pin;
use std::cell::RefCell;
use std::rc::Rc;
use actix_service::{Service, Transform};
// use actix_web::web::json;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, HttpResponse, Error};
use futures::future::{ok, Ready};
use futures::Future;
use actix_session::UserSession;
use crate::AppError;

use jsonwebtoken::Validation;
use jsonwebtoken::DecodingKey;
use crate::utils::SessionClaims;
use jsonwebtoken::{decode, Algorithm};
use log::{info};


#[derive(Debug, Clone)]
pub struct Authentication;

impl<S> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_service::forward_ready!(service);

    #[allow(clippy::borrow_interior_mutable_const)]
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        Box::pin(async move {
            let session = req.get_session();
            let bearer = match req.headers().get("Authorization") {
                Some(b) => b.to_str(),
                None => {
                    return Err(AppError::from("Header not found: Authorization").into()); 
                }
            };
            let bearer = match bearer {
                Ok(b) => b,
                Err(err) => {
                    return Err(AppError::from(err.to_string()).into()); 
                }
            };
            let bearer = if bearer.len() < 8 {
                return Err(AppError::from("Invalid token from length.").into()); 
            } else {
                &bearer[7..]
            };
            let token_claims = decode::<SessionClaims>(
                bearer,
                &DecodingKey::from_secret("secret".as_bytes()),
                &Validation::new(Algorithm::HS512),
            );
            let token_claims = match token_claims {
                Ok(t) => t,
                Err(_) => {
                    return Ok(req.into_response(HttpResponse::Unauthorized()));
                }
            };
            let session_token = match session.get::<String>(&token_claims.claims.get_id()) { 
                Ok(s) => {
                    if s.is_none() {
                        return Err(AppError::from("Server error. Token not found.").into()); 
                    }
                    s.unwrap()
                },
                Err(err) => {
                    return Err(AppError::from(err).into()); 
                }
            };
            if session_token != bearer {
                return Ok(req.into_response(HttpResponse::Unauthorized().body("Somone fuckin touched the token.").into_body()));
            }
            
            session.renew();
            info!("Auth request");
            let res_fut = srv.call(req);
            res_fut.await
        })
    }
}