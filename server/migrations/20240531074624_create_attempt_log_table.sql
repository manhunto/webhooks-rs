CREATE TABLE attempt_logs
(
    message_id char(27) NOT NULL,
    attempt SMALLINT NOT NULL,
    primary key(message_id, attempt),
    processing_time INT NOT NULL,
    response_time INT NOT NULL,
    response_body TEXT NULL
);
