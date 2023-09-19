use sankar;

DROP TABLE IF EXISTS book;
DROP TABLE IF EXISTS books;
DROP TABLE IF EXISTS userbooks;
DROP TABLE IF EXISTS categorybooks;
DROP TABLE IF EXISTS updatebooknode;

CREATE TABLE books (
    docid timeuuid,
    authorId timeuuid,
    title varchar,
    body text,
    url text,
    metadata text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (docid, createdAt)
) WITH CLUSTERING ORDER BY (createdAt DESC);
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
) WITH CLUSTERING ORDER BY (uniqueId DESC);
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
) WITH CLUSTERING ORDER BY (docid DESC);
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
) WITH CLUSTERING ORDER BY (docid DESC);
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
CREATE TABLE book_title (
    docid timeuuid,
    title text,
    parentId timeuuid,
    uniqueId timeuuid,
    identity smallint,
    PRIMARY KEY (docid, uniqueId)
) WITH CLUSTERING ORDER BY (uniqueId DESC);

SELECT COUNT(*) FROM books;


TRUNCATE TABLE sankar.books;
TRUNCATE TABLE sankar.book;
TRUNCATE TABLE sankar.userbooks;
TRUNCATE TABLE sankar.categorybooks;

ALTER TABLE users
  ADD COLUMN pwd varchar(225) NOT NULL,
  ADD COLUMN email varchar(50) NOT NULL,
  ADD COLUMN createdAt TIMESTAMP NOT NULL,
  ADD COLUMN updatedAt TIMESTAMP NOT NULL;

CREATE TABLE users (
    userId serial PRIMARY KEY,
    fname varchar(50) NOT NULL,
    lname varchar(50),
    email varchar(50) UNIQUE NOT NULL,
    pwd varchar(255) NOT NUll,
    createdAt TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updatedAt TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE users
  ALTER COLUMN createdAt TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  ALTER COLUMN updatedAt TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP;

