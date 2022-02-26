mod create;
mod read;
mod delete;
mod image;
mod queries;
mod any;

pub use image::upload_image;
pub use create::{parent_request, child_request};
pub use create::create_and_update::create_and_update_chapter;
pub use read::{get_all, get_one, get_all_from_id};
pub use delete::update_or_delete;
pub use any::any;