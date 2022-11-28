USE movienight;

CREATE TABLE IF NOT EXISTS users
(
    id               INTEGER AUTO_INCREMENT PRIMARY KEY,
    name             VARCHAR(255) NOT NULL,
    email            VARCHAR(255) NOT NULL UNIQUE,
    password         VARCHAR(255) NOT NULL,
    username         VARCHAR(255) NOT NULL,
    token            VARCHAR(255) UNIQUE,
    token_expiration DATETIME,
    created_at   DATETIME NOT NULL,
    updated_at   DATETIME NOT NULL,
    deleted_at   DATETIME
);
