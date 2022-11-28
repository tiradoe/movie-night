USE movienight;

CREATE TABLE IF NOT EXISTS schedules
(
    id        INTEGER AUTO_INCREMENT PRIMARY KEY,
    name      VARCHAR(255),
    is_public BOOLEAN,
    slug      VARCHAR(255) UNIQUE,
    owner     INTEGER,
    created_at   DATETIME NOT NULL,
    updated_at   DATETIME NOT NULL,
    deleted_at   DATETIME,
    FOREIGN KEY (owner) REFERENCES users (id)
);
