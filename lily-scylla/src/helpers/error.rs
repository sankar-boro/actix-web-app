use serde::Serialize;

#[derive(Serialize)]
pub struct Error<'a> {
    status: &'a str,
    message: &'a str,
    data: Option<String>
}

impl<'a> Error<'a> {
    pub fn not_found(message: &'a str) -> Self {
        Self {
            status: "NOT_FOUND",
            message,
            data: None,
        }
    }

    pub fn db_error(message: &'a str) -> Self {
        Self {
            status: "DB_ERROR",
            message,
            data: None,
        }
    }
}