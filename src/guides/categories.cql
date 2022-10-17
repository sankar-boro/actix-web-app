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


TRUNCATE TABLE sankar.usercategories;