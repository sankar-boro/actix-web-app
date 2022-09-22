pub static CREATE_BOOK_NODE_QUERY: &str = "INSERT INTO sankar.book (
    bookId, uniqueId, parentId, authorId, title, body, identity, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_BLOG_NODE_QUERY: &str = "INSERT INTO sankar.blog (
    blogId, uniqueId, parentId, title, body, identity, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_BOOKS: &str = "INSERT INTO sankar.books (
    bookId, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";
/**
 * We dont include parentId, because the first node is the parent node.
 */
pub static CREATE_BOOK: &str = "INSERT INTO sankar.book (
    bookId, uniqueId, authorId, title, body, url, identity, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)";
pub static CREATE_USER_BOOKS: &str = "INSERT INTO sankar.userbooks (
    bookId, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_CATEGORY_BOOKS: &str = "INSERT INTO sankar.categorybooks (
    category, bookId, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?
)";

// pub static CREATE_USER_CATEGORIES: &str = "INSERT INTO sankar.usercategories (
//     authorId, category, createdAt, updatedAt
// ) VALUES(
//     ?, ?, ?, ?
// )";

pub static ADD_CATEGORY: &str = "INSERT INTO sankar.usercategories (
    authorId, category, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?
)";

pub static DELETE_CATEGORY: &str = "DELETE FROM sankar.usercategories WHERE authorId=? AND category=?";

// pub static CREATE_USER_SESSION: &str = "INSERT INTO sankar.session (
//     userId, sessionId, createdAt, updatedAt
// ) VALUES(
//     ?, ?, ?, ?
// )";

pub static CREATE_BLOGS: &str = "INSERT INTO sankar.blogs (
    blogId, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";
pub static CREATE_BLOG: &str = "INSERT INTO sankar.blog (
    blogId, uniqueId, authorId, title, body, url, identity, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)";
pub static CREATE_USER_BLOGS: &str = "INSERT INTO sankar.userblogs (
    blogId, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_CATEGORY_BLOGS: &str = "INSERT INTO sankar.categoryblogs (
    category, blogId, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_ALLCATEGORY: &str = "INSERT INTO sankar.allcategories (
    category, division
) VALUES(
    ?, ?
) IF NOT EXISTS";