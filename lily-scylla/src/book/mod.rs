mod create;
// mod schema;
// mod db;
mod read;
mod update;
mod delete;
mod image;

pub use image::upload_image;
pub use create::{create_new_book, create_new_chapter, create_new_section, create_new_page, create_and_update_chapter};
pub use read::{get_all, get_one, get_all_from_id};
pub use update::update_one;
// pub use delete::{delete_section_last, delete_section_first, delete_main_section, delete_many};
pub use delete::update_or_delete;