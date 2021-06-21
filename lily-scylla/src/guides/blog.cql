CREATE TABLE blog (
    blogId timeuuid,
    title varchar,
    tags varchar,
    summary text,
    body text,
    authorId timeuuid,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (blogId, createdAt)
);
