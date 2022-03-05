pub static CREATE_NODE_QUERY: &str = "INSERT INTO sankar.book (
    bookId, uniqueId, parentId, title, body, identity, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";
