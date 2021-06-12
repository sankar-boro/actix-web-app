use std::pin::Pin;
use std::cell::RefCell;
use std::rc::Rc;
use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, HttpResponse, Error};
use futures::future::{ok, Ready};
use futures::Future;
use actix_session::UserSession;
use crate::App;

use super::validation::ValidationHandler;



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
        let session = req.get_session();
        let srv = self.service.clone();
        let bearer = req.headers().get("Authorization");
        let auth = ValidationHandler::new(bearer).verify_token(&session);
        Box::pin(async move {
            if auth.is_err()
            {
                Ok(req.into_response(HttpResponse::Unauthorized().body("UnAuthorized Error").into_body()))
            } else {
                let res_fut = srv.call(req);
                res_fut.await
            }
        })
    }
}

