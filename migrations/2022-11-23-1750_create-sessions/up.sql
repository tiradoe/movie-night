USE movienight;

CREATE TABLE IF NOT EXISTS sessions
(
    id         INTEGER UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    user_id    INTEGER      NOT NULL,
    token      VARCHAR(255) NOT NULL UNIQUE,
    FOREIGN KEY (user_id) REFERENCES users (id),
    expiration DATETIME     NOT NULL
);
