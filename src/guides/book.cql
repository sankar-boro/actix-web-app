CREATE TABLE book (
    bookId timeuuid,
    uniqueId timeuuid,
    parentId timeuuid,
    authorId timeuuid,
    authorName varchar,
    identity smallint,
    title varchar,
    body text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (bookId, uniqueId)
);