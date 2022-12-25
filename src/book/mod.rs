mod read;
mod user;
mod titles;

pub use read::{
    getBooksWithPageSize,
    getNextBooksWithPageSize,
    getBookNodesWithPageSizeFromId,
    getNextBookNodesWithPageSizeFromId,
    getBooksWithPageSizeCategories,
    getBooksWithPageSizeCategoriesNext
};
pub use user::{
    getPagedBooksForAuthorId, 
    getPagedBlogsForAuthorId,
    getNextPageBooksForAuthorId, 
    getNextPageBlogsForAuthorId,
};

pub use titles::get_titles;