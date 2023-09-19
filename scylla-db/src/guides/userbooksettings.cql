use sankar;

DROP TABLE IF EXISTS userbooksettings;

CREATE TABLE userbooksettings (
    authorId timeuuid,
    docid timeuuid,
    settings text,
    PRIMARY KEY (authorId, docid)
);

TRUNCATE TABLE sankar.userbooksettings;
