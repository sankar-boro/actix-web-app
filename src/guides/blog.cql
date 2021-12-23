CREATE TABLE blog (
    blogId timeuuid,
    authorId timeuuid,
    authorName varchar,
    title varchar,
    body text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (blogId)
);