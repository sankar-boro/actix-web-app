use sankar;
DROP TABLE IF EXISTS blog;
DROP TABLE IF EXISTS blogInfo;

CREATE TABLE blog (
    blogId timeuuid,
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
    PRIMARY KEY (blogId, uniqueId)
);

CREATE TABLE blogInfo (
    blogId timeuuid,
    authorId timeuuid,
    fname varchar,
    lname varchar,
    title varchar,
    body text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (blogId)
);

INSERT INTO sankar.blogInfo (
    blogId, authorId, fname, lname, title, body, createdAt, updatedAt
) VALUES(
    $1, $2, $3, $4, $5, $6, $7
)