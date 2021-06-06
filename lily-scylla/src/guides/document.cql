CREATE TABLE documents (
    documentId timeuuid,
    title varchar,
    tags varchar,
    body varchar,
    PRIMARY KEY (documentId)
);


    -- smDocumentImageUrl text,
    -- mdDocumentImageUrl text,
    -- lgDocumentImageUrl text,
    -- authorId timeuuid,
    -- authorName varchar,
    -- authorImageUrl text,  
    -- createdAt timeuuid,
    -- updatedAt timeuuid,

CREATE TABLE documentsByUser (
    documentId timeuuid,
    title text,
    tags text,
    body text,
    smDocumentImageUrl text,
    mdDocumentImageUrl text,
    lgDocumentImageUrl text,
    authorId timeuuid,
    authorName varchar,
    authorImageUrl text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (authorId, documentId)
) WITH CLUSTERING ORDER BY (documentId DESC);

CREATE TABLE documentsByTags (
    documentId timeuuid,
    title text,
    tag text,
    body text,
    smDocumentImageUrl text,
    mdDocumentImageUrl text,
    lgDocumentImageUrl text,
    authorId timeuuid,
    authorName varchar,
    authorImageUrl text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (tag, documentId)
) WITH CLUSTERING ORDER BY (documentId DESC);