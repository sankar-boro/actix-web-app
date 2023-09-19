use sankar;

DROP TABLE IF EXISTS blog;
DROP TABLE IF EXISTS blogs;
DROP TABLE IF EXISTS userblogs;
DROP TABLE IF EXISTS categoryblogs;

CREATE TABLE blogs (
    docid timeuuid,
    authorId int,
    title varchar,
    body text,
    url text,
    metadata text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (docid)
);
CREATE TABLE blog (
    docid timeuuid,
    uniqueId timeuuid,
    parentId timeuuid,
    authorId int,
    title varchar,
    body text,
    url text,
    identity smallint,
    metadata text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (docid, uniqueId)
);
CREATE TABLE userblogs (
    docid timeuuid,
    authorId int,
    title varchar,
    body text,
    url text,
    metadata text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (authorId, docid)
);
CREATE TABLE categoryblogs (
    category text,
    docid timeuuid,
    authorId int,
    title varchar,
    body text,
    url text,
    metadata text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (category, docid)
);

TRUNCATE TABLE sankar.blogs;
TRUNCATE TABLE sankar.blog;
TRUNCATE TABLE sankar.userblogs;
TRUNCATE TABLE sankar.categoryblogs;

SELECT COUNT(*) FROM blogs;
