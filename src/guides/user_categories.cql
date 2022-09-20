use sankar;
DROP TABLE IF EXISTS usercategories;
CREATE TABLE usercategories (
    authorId timeuuid,
    category varchar,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (authorId)
);