use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
    status: &'static str,
    message: &'static str,
    data: Option<String>
}

impl Error {
    pub fn not_found(message: &'static str) -> Self {
        Self {
            status: "NOT_FOUND",
            message,
            data: None,
        }
    }

    pub fn db_error(message: &'static str) -> Self {
        Self {
            status: "DB_ERROR",
            message,
            data: None,
        }
    }
}