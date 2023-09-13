#![allow(unused_imports, dead_code)]

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

/* Book */

pub static BOOK_DATA: &str = "SELECT uid, authorid, bookid, parentid, title, body, identity, metadata FROM booknode WHERE bookid=$1";

pub static CREATE_BOOK: &str = "INSERT INTO book (
    authorid, title, body, imageurl, metadata
) VALUES(
    $1, $2, $3, $4, $5
) RETURNING uid";

pub static CREATE_BOOK_TITLE: &str = "INSERT INTO title (
    bookid, parentid, title, identity
) VALUES(
    $1, $2, $3, $4
)";

pub static CREATE_BOOK_NODE: &str = "INSERT INTO booknode (
    authorid, bookid, parentid, title, body, imageurl, identity, metadata
) VALUES(
    $1, $2, $3, $4, $5, $6, $7, $8
)";

pub static UPDATE_BOOKS: &str = "UPDATE books SET title=$1, body=$2, metadata=$3 WHERE uid=$4";

pub static DELETE_BOOKS: &str = "DELETE FROM books where uid=$1";
/* Book */


/* Blog */

pub static BLOG_DATA: &str = "SELECT uid, authorid, blogid, parentid, title, body, identity, metadata FROM blognode WHERE blogid=$1";

pub static CREATE_BLOG: &str = "INSERT INTO blog (
    authorid, title, body, imageurl, metadata
) VALUES(
    $1, $2, $3, $4, $5
) RETURNING uid";

pub static CREATE_BLOG_NODE: &str = "INSERT INTO blognode (
    authorid, blogid, parentid, title, body, imageurl, identity, metadata
) VALUES(
    $1, $2, $3, $4, $5, $6, $7, $8
)";

pub static UPDATE_BLOGS: &str = "UPDATE blogs SET title=$1, body=$2, metadata=$3 WHERE uid=$4";

pub static DELETE_BLOGS: &str = "DELETE FROM blogs where uid=$1";

/* Blog */


/**
 * We dont include parentId, because the first node is the parent node.
 */


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
pub static ADD_USER_CATEGORY: &str = "INSERT INTO sankar.usercategories (
    authorId, category, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?
) IF NOT EXISTS";
pub static DELETE_CATEGORY: &str = "DELETE FROM sankar.usercategories WHERE authorId=? AND category=?";
pub static CREATE_BLOGS: &str = "INSERT INTO sankar.blogs (
    blogId, authorId, title, body, url, metadata, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?, ?, ?, ?, ?
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
    category, authorId, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?
) IF NOT EXISTS";

pub static CREATE_USER_BOOK_SETTINGS: &str = "INSERT INTO sankar.userbooksettings (
    authorId, bookId, settings
) VALUES(
    ?, ?, ?
)";

pub static UPDATE_USER_BOOK_SETTINGS: &str = "UPDATE sankar.userbooksettings SET settings=? where authorId=? AND bookId=?";

pub static FOLLOW_USER: &str = "INSERT INTO sankar.followers (
    userId, followerId, createdAt, updatedAt
) VALUES(
    ?, ?, ?, ?
) IF NOT EXISTS";
pub static UNFOLLOW_USER: &str = "DELETE FROM sankar.followers WHERE userId=? AND followerId=?";

pub static DELETE_USERBOOKS: &str = "DELETE FROM sankar.userbooks where authorId=? AND bookId IN (?)";

pub static SIGNUP: &str = "INSERT INTO users (fname, lname, email, password) VALUES ($1, $2, $3, $4) RETURNING uid";