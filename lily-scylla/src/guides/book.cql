CREATE TABLE books (
    bookId timeuuid,
    title varchar,
    authorId timeuuid,
    description text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (bookId)
);

INSERT INTO books (
    bookId,
    title,
    authorId,
    description,
    createdAt,
    updatedAt
) VALUES (
    4c8ef908-d187-11eb-b8bc-0242ac130003,
    'Gemini',
    a8f856b9-d183-11eb-b423-2de8529f1fd5,
    'Get to know and understand the two sided characteristics, features of Gemini.',
    4c8ef908-d187-11eb-b8bc-0242ac130003,
    4c8ef908-d187-11eb-b8bc-0242ac130003
);
INSERT INTO books (
    bookId,
    title,
    authorId,
    description,
    createdAt,
    updatedAt
) VALUES (
    86338792-d1c2-11eb-b8bc-0242ac130003,
    'Aries',
    a8f856b9-d183-11eb-b423-2de8529f1fd5,
    'Get to know and understand the fearless Aries. Mother of all.',
    86338792-d1c2-11eb-b8bc-0242ac130003,
    86338792-d1c2-11eb-b8bc-0242ac130003
);




CREATE TABLE book (
    bookId timeuuid,
    uniqueId timeuuid,
    parentId timeuuid,
    pageNo smallint,
    identity smallint,
    title varchar,
    body text,
    createdAt timeuuid,
    updatedAt timeuuid,
    PRIMARY KEY (bookId, uniqueId)
);

BEGIN BATCH
INSERT INTO book (
    bookId,
    uniqueId,
    parentId,
    identity,
    title,
    body
) VALUES (
    4c8ef908-d187-11eb-b8bc-0242ac130003,
    fffdb392-d22e-11eb-b8bc-0242ac130003,
    4c8ef908-d187-11eb-b8bc-0242ac130003,
    01,
    'TYPICAL GEMINI PERSONALITY TRAITS',
    'To get an idea of the classic Gemini personality traits and types, 
    we need only to look to some of the most famous twins (the star sign’s symbol): 
    Kanye West, Kylie Minogue, Tupac, Marilyn Monroe, 
    Angelina Jolie and Donald Trump (sorry) to name a few. These guys ALL have big personalities.'
);
INSERT INTO book (
    bookId,
    uniqueId,
    parentId,
    identity,
    title,
    body
) VALUES (
    4c8ef908-d187-11eb-b8bc-0242ac130003,
    bbf28f78-d22f-11eb-b8bc-0242ac130003,
    fffdb392-d22e-11eb-b8bc-0242ac130003,
    02,
    'VERSATILE',
    'If the phrase "Jack of all trades, master of none" applied to any 
    sign then it’s Gemini. You will, at first, think your Gemmy 
    friend is super clever, like, how do they fucking KNOW ALL OF 
    THIS STUFF? Well, it’s not as clever as it looks. 
    They skim-read and acquire titbits of knowledge, 
    like the Mercurial magpie they are, BUT it only ever reaches skin-deep. 
    Their interest rarely lingers long enough for them to truly delve into any one subject. 
    A bit of knowledge… and they’re off to the next shiny gem.'
);
INSERT INTO book (
    bookId,
    uniqueId,
    parentId,
    identity,
    title,
    body
) VALUES (
    4c8ef908-d187-11eb-b8bc-0242ac130003,
    ff30e654-d22f-11eb-b8bc-0242ac130003,
    fffdb392-d22e-11eb-b8bc-0242ac130003,
    02,
    'SMART',
    'Feelings = ugh. Geminis are not known for being sentimental or emotional, 
    they are the kind of friend who will disappear if you start crying 
    (unless you’re also simultaneously revealing some really juicy gossip). 
    Their leaning on the analytical, mental, cerebral side of life makes it easier 
    for them to disconnect and see situations dispassionately. They adore dissecting 
    thorny issues but they’re likely to come up with some pretty, quite frankly, psychopathic-sounding solutions.'
);
INSERT INTO book (
    bookId,
    uniqueId,
    parentId,
    identity,
    title,
    body
) VALUES (
    4c8ef908-d187-11eb-b8bc-0242ac130003,
    7a9877b2-d230-11eb-b8bc-0242ac130003,
    fffdb392-d22e-11eb-b8bc-0242ac130003,
    02,
    'YOUTHFUL',
    'Happily for them, they tend to be the Peter Pan of the zodiac - both physically and mentally. 
    They are still into the latest trends long after everyone else has “settled down” 
    (Geminis hate the idea of “settling down”, sounds dangerously like “dying” to them).
    The flip side of looking young and being so perky can mean they are also flaky, childish and silly at times. 
    They also party WAY too much.'
);
APPLY BATCH;

-- ALTER TABLE sankar.book RENAME type TO secType;


-- BEGIN BATCH
--   INSERT INTO cycling.cyclist_expenses (cyclist_name, balance) VALUES ('Vera ADRIAN', 0) IF NOT EXISTS;
--   INSERT INTO cycling.cyclist_expenses (cyclist_name, expense_id, amount, description, paid) VALUES ('Vera ADRIAN', 1, 7.95, 'Breakfast', false);
--   APPLY BATCH;