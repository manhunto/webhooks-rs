CREATE TABLE endpoints
(
    id     char(27)  NOT NULL,
    primary key (id),
    app_id char(27)  NOT NULL,
    url    TEXT      NOT NULL,
    topics JSON      NOT NULL,
    status char(127) NOT NULL
);