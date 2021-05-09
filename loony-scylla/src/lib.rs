pub mod route;

use std::sync::Arc;

use scylla::{Session, SessionBuilder};
use actix_web::{App as ActixApp, HttpServer};
use scylla::transport::session::{IntoTypedRows};

#[derive(Clone)]
pub struct App {
    conn: Arc<Session>,
}

impl App {
    pub fn new(conn: Arc<Session>) -> Self {
        Self {
            conn,
        }
    }
}