#[allow(dead_code)]

use serde::Serialize;

#[derive(Serialize)]
pub struct Error<'a> {
    status: &'a str,
    message: &'a str,
    data: Option<String>
}
impl<'a> Error<'a> {

    #[allow(dead_code)]
    pub fn not_found(message: &'a str) -> Self {
        Self {
            status: "NOT_FOUND",
            message,
            data: None,
        }
    }

    #[allow(dead_code)]
    pub fn db_error(message: &'a str) -> Self {
        Self {
            status: "DB_ERROR",
            message,
            data: None,
        }
    }

    #[allow(dead_code)]
    pub fn bad_request(message: &'a str) -> Self {
        Self {
            status: "BAD_REQUEST",
            message,
            data: None,
        }
    }
}