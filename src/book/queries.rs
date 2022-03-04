pub static CHILD: &str = "INSERT INTO sankar.book (
    bookId, uniqueId, parentId, title, body, identity, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static UPDATE_PARENT_ID: &str = "UPDATE sankar.book SET parentId=? WHERE bookId=? AND uniqueId=?";
// pub static UPDATE_BOOK: &str = "UPDATE sankar.book SET title=?, body=? WHERE bookId=? AND uniqueId=?";