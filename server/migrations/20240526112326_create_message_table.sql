CREATE TABLE messages
(
    id     char(27)  NOT NULL,
    primary key (id),
    event_id char(27)  NOT NULL,
    endpoint_id char(27)  NOT NULL
);

CREATE TABLE attempts
(
    message_id char(27) NOT NULL,
    attempt SMALLINT NOT NULL,
    primary key(message_id, attempt),
    status_numeric SMALLINT NULL,
    status_unknown TEXT NULL
);
