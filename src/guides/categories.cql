use sankar;
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
    PRIMARY KEY (category)
);