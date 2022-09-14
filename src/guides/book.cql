use sankar;

DROP TABLE IF EXISTS book;
DROP TABLE IF EXISTS bookInfo;
DROP TABLE IF EXISTS userBooks;

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
    url text,
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
    url text,
    metadata text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (bookId)
);

CREATE TABLE userBooks (
    bookId timeuuid,
    uniqueId timeuuid,
    parentId timeuuid,
    authorId timeuuid,
    fname varchar,
    lname varchar,
    identity smallint,
    title varchar,
    body text,
    url text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (authorId)
);