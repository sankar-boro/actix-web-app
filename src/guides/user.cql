CREATE TABLE users (
    userId timeuuid,
    fname varchar,
    lname varchar,
    smUserImageUrl varchar,
    mdUserImageUrl varchar,
    lgUserImageUrl varchar,
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
    smUserImageUrl varchar,
    mdUserImageUrl varchar,
    lgUserImageUrl varchar,
    email varchar,
    password blob,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (email)
);
