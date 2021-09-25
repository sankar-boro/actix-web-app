use std::pin::Pin;
use std::cell::RefCell;
use std::rc::Rc;
use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use futures::Future;
use actix_session::UserSession;
use crate::AppError;
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
        info!("AUTH_REQUEST");
        let srv = self.service.clone();
        Box::pin(async move {
            let session = req.get_session();
            let user_id = match session.get::<String>("AUTH_ID") { 
                Ok(s) => s,
                Err(err) => {
                    return Err(AppError::from(err).into()); 
                }
            };
            match user_id {
                Some(_) => {},
                None => return Err(AppError::from("UN_AUTHENTICATED_USER").into())
            }
            let res_fut = srv.call(req);
            res_fut.await
        })
    }
}