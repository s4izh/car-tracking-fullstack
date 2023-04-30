-- Your SQL goes here
CREATE TABLE users (
    id              INT             NOT NULL AUTO_INCREMENT,
    matricula       VARCHAR(255)    NOT NULL UNIQUE,
    hash            VARCHAR(255)    NOT NULL,
    total_km        INT             NOT NULL DEFAULT 0,
    date_created    TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (id)
);
