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
    metadata text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (blogId)
);