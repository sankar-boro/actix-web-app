pub static CREATE_BOOK_NODE_QUERY: &str = "INSERT INTO sankar.book (
    bookId, uniqueId, parentId, title, body, identity, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_BLOG_NODE_QUERY: &str = "INSERT INTO sankar.blog (
    blogId, uniqueId, parentId, title, body, identity, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_USER_SESSION: &str = "INSERT INTO sankar.session (
    userId, sessionId, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?
)";