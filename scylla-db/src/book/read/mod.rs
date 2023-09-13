mod readBook;
mod subscribedDocs;

pub use readBook::*;
pub use subscribedDocs::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NextPageRequest {
    page: Vec<u8>
}