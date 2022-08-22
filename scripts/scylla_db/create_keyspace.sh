#!/bin/bash

CREATE KEYSPACE sankar WITH replication = {'class':'SimpleStrategy', 'replication_factor' : 1 };
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