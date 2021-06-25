CREATE TABLE book (
    bookId timeuuid,
    uniqueId timeuuid,
    parentId timeuuid,
    authorId timeuuid,
    authorName varchar,
    identity smallint,
    title varchar,
    body text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (bookId, uniqueId)
);


-- ALTER TABLE sankar.book RENAME type TO secType;


-- BEGIN BATCH
--   INSERT INTO cycling.cyclist_expenses (cyclist_name, balance) VALUES ('Vera ADRIAN', 0) IF NOT EXISTS;
--   INSERT INTO cycling.cyclist_expenses (cyclist_name, expense_id, amount, description, paid) VALUES ('Vera ADRIAN', 1, 7.95, 'Breakfast', false);
--   APPLY BATCH;


-- IDENTITY
-- 101 FRONT COVER
-- 102 BACK COVER
-- 103 SINGLE PAGE
-- 104 CHAPTER
-- 104 SECTION
-- 105 SUB-SECTION