CREATE TABLE session (
    userId timeuuid,
    sessionId timeuuid,
    body text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (userId)
);