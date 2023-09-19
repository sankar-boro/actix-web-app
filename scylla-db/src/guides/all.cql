use sankar;

DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS userCredentials;

CREATE TABLE users (
    userId timeuuid,
    fname varchar,
    lname varchar,
    email varchar,
    password blob,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (userId)
);

CREATE TABLE userCredentials (
    userId timeuuid,
    fname varchar,
    lname varchar,
    email varchar,
    password blob,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (email)
);

DROP TABLE IF EXISTS blog;
DROP TABLE IF EXISTS blogs;
DROP TABLE IF EXISTS userblogs;
DROP TABLE IF EXISTS categoryblogs;

CREATE TABLE blogs (
    docid timeuuid,
    authorId timeuuid,
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
    authorId timeuuid,
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
    authorId timeuuid,
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
    authorId timeuuid,
    title varchar,
    body text,
    url text,
    metadata text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (category, docid)
);

DROP TABLE IF EXISTS book;
DROP TABLE IF EXISTS books;
DROP TABLE IF EXISTS userbooks;
DROP TABLE IF EXISTS categorybooks;

CREATE TABLE books (
    docid timeuuid,
    authorId timeuuid,
    title varchar,
    body text,
    url text,
    metadata text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (docid)
);
CREATE TABLE book (
    docid timeuuid,
    uniqueId timeuuid,
    parentId timeuuid,
    authorId timeuuid,
    title varchar,
    body text,
    url text,
    identity smallint,
    metadata text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (docid, uniqueId)
);
CREATE TABLE userbooks (
    docid timeuuid,
    authorId timeuuid,
    title varchar,
    body text,
    url text,
    metadata text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (authorId, docid)
);
CREATE TABLE categorybooks (
    category text,
    docid timeuuid,
    authorId timeuuid,
    title varchar,
    body text,
    url text,
    metadata text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (category, docid)
);
CREATE TABLE updatebooknode (
    docid timeuuid,
    uniqueId timeuuid,
    parentId timeuuid,
    authorId timeuuid,
    body text,
    identity smallint,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY ((docid, parentId), uniqueId)
) WITH CLUSTERING ORDER BY (uniqueId DESC);

DROP TABLE IF EXISTS usercategories;
DROP TABLE IF EXISTS allcategories;

CREATE TABLE usercategories (
    authorId timeuuid,
    category varchar,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (authorId, category)
);
CREATE TABLE allcategories (
    category varchar,
    division varchar,
    PRIMARY KEY (category, division)
);
INSERT INTO sankar.allcategories (category, division) VALUES('science', 'demo');
INSERT INTO sankar.allcategories (category, division) VALUES('physics', 'demo');
INSERT INTO sankar.allcategories (category, division) VALUES('chemistry', 'demo');
INSERT INTO sankar.allcategories (category, division) VALUES('maths', 'demo');
INSERT INTO sankar.allcategories (category, division) VALUES('biology', 'demo');
INSERT INTO sankar.allcategories (category, division) VALUES('history', 'demo');
INSERT INTO sankar.allcategories (category, division) VALUES('assamese', 'demo');
INSERT INTO sankar.allcategories (category, division) VALUES('bodo', 'demo');
INSERT INTO sankar.allcategories (category, division) VALUES('computer', 'demo');
INSERT INTO sankar.allcategories (category, division) VALUES('literature', 'demo');
INSERT INTO sankar.allcategories (category, division) VALUES('social science', 'demo');
