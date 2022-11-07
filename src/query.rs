#[macro_export]
macro_rules! create_query {
    ( $a:expr, $( $x:expr ),* ) => {
        {
            let xx = format!("INSERT INTO {}", $a);
            let mut aa = String::from("(");
            let mut bb = String::from("VALUES (");
            $(
                aa.push_str($x);
                aa.push_str(", ");
                bb.push_str("?, ");
            )*
            let mut aa = format!("{}", &aa[0..aa.len()-2]);
            let mut bb = format!("{}", &bb[0..bb.len()-2]);
            aa.push_str(")");
            bb.push_str(")");
            format!("{} {} {}", xx, aa, bb)
        }
    };
}

pub static CREATE_BOOK_NODE_QUERY: &str = "INSERT INTO sankar.book (
    bookId, uniqueId, parentId, authorId, title, body, metadata, url, identity, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)";

pub static CREATE_BLOG_NODE_QUERY: &str = "INSERT INTO sankar.blog (
    blogId, uniqueId, parentId, authorId, title, body, metadata, url, identity, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
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

pub static CREATE_USER_BOOK_SETTINGS: &str = "INSERT INTO sankar.userbooksettings (
    authorId, bookId, settings
) VALUES(
    ?, ?, ?
)";

pub static UPDATE_USER_BOOK_SETTINGS: &str = "UPDATE sankar.userbooksettings SET settings=? where authorId=? AND bookId=?";

pub static GET_SIZE: i32 = 8;
pub static PAGE_SIZE: i32 = 30;