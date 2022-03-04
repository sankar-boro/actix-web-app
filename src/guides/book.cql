use sankar;
DROP TABLE IF EXISTS book;
DROP TABLE IF EXISTS bookInfo;

CREATE TABLE book (
    bookId timeuuid,
    uniqueId timeuuid,
    parentId timeuuid,
    authorId timeuuid,
    fname varchar,
    lname varchar,
    identity smallint,
    title varchar,
    body text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (bookId, uniqueId)
);

CREATE TABLE bookInfo (
    bookId timeuuid,
    authorId timeuuid,
    fname varchar,
    lname varchar,
    title varchar,
    body text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (bookId)
);

INSERT INTO sankar.bookInfo (
    bookId, authorId, fname, lname, title, body, createdAt, updatedAt
) VALUES(
    $1, $2, $3, $4, $5, $6, $7
)