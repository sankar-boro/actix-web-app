mod create;
mod read;
mod update;
mod delete;
mod image;
mod queries;

pub use image::upload_image;
pub use create::new_book::create_new_book;
pub use create::new_chapter::create_new_chapter;
pub use create::new_page::create_new_page;
pub use create::new_section::create_new_section;
pub use create::create_and_update_chapter::create_and_update_chapter;
pub use read::{get_all, get_one, get_all_from_id};
pub use update::update_one;
pub use delete::update_or_delete;