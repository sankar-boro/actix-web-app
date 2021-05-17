CREATE TABLE documents (
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
    PRIMARY KEY (documentId)
);

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