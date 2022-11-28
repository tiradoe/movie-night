CREATE DATABASE IF NOT EXISTS movienight;
USE movienight;

CREATE TABLE IF NOT EXISTS movies
(
    id           INTEGER AUTO_INCREMENT PRIMARY KEY,
    title        TEXT     NOT NULL,
    imdb_id      VARCHAR(255),
    year         SMALLINT UNSIGNED,
    critic_score VARCHAR(255),
    genre        VARCHAR(255),
    director     VARCHAR(255),
    actors       VARCHAR(255),
    plot         TEXT,
    poster       VARCHAR(500),
    is_good      BOOLEAN,
    added_by     VARCHAR(255),
    last_watched DATETIME,
    created_at   DATETIME NOT NULL,
    updated_at   DATETIME NOT NULL,
    deleted_at   DATETIME
);
