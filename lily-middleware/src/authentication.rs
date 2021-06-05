use std::pin::Pin;
use std::task::{Context, Poll};
use std::cell::RefCell;
use std::rc::Rc;
use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};
use futures::future::{ok, Ready};
use futures::Future;
use actix_session::UserSession;
use super::validation::ValidationHandler;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
#[derive(Clone)]
pub struct AuthService;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for AuthService
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service: Rc::new(RefCell::new(service)) })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();
        let mut srv = self.service.clone();
        let bearer = req.headers().get("Authorization");
        let auth = ValidationHandler::new(bearer).verify_token(&session);
        Box::pin(async move {
            if auth.is_err()
            {
                Ok(req.into_response(HttpResponse::Unauthorized().body("UnAuthorized Request").into_body()))
            } else {
                let res_fut = srv.call(req);
                res_fut.await
            }
        })
    }
}

