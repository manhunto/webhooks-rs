CREATE TABLE events
(
    id     char(27)  NOT NULL,
    primary key (id),
    app_id char(27)  NOT NULL,
    payload JSON      NOT NULL,
    topic text NOT NULL,
    created_at TIMESTAMP NOT NULL
);
