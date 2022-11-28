USE movienight;

CREATE TABLE IF NOT EXISTS movielist_movie
(
    id           INTEGER AUTO_INCREMENT PRIMARY KEY,
    movielist_id INTEGER  NOT NULL,
    movie_id     INTEGER  NOT NULL,
    created_at   DATETIME NOT NULL,
    updated_at   DATETIME NOT NULL,
    deleted_at   DATETIME,
    FOREIGN KEY (movie_id) REFERENCES movies (id),
    FOREIGN KEY (movielist_id) REFERENCES movie_lists (id),
    unique (movielist_id, movie_id)
);
