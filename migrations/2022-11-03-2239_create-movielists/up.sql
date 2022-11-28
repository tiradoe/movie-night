USE movienight;

CREATE TABLE IF NOT EXISTS movie_lists
(
    id         INTEGER AUTO_INCREMENT PRIMARY KEY,
    name       TEXT             NOT NULL,
    public     BOOLEAN          NOT NULL,
    owner      INTEGER          NOT NULL,
    created_at DATETIME         NOT NULL,
    updated_at DATETIME         NOT NULL,
    deleted_at DATETIME,
    FOREIGN KEY (owner) REFERENCES users (id)
);
