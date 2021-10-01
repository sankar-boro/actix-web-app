pub static PARENT: &str = "INSERT INTO sankar.book (
    bookId, uniqueId, authorId, authorName, title, body, identity, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CHILD: &str = "INSERT INTO sankar.book (
    bookId, uniqueId, parentId, title, body, identity, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static UPDATE: &str = "UPDATE sankar.book SET parentId=? WHERE bookId=? AND uniqueId=?";