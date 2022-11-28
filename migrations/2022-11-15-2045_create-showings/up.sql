USE movienight;

CREATE TABLE IF NOT EXISTS showings
(
    id          INTEGER AUTO_INCREMENT PRIMARY KEY,
    schedule_id INTEGER,
    movie_id    INTEGER,
    showtime    DATETIME,
    public      BOOL default FALSE,
    owner       INTEGER NOT NULL,
    created_at   DATETIME NOT NULL,
    updated_at   DATETIME NOT NULL,
    deleted_at   DATETIME,
    FOREIGN KEY (movie_id) REFERENCES movies (id),
    FOREIGN KEY (schedule_id) REFERENCES schedules (id),
    FOREIGN KEY (owner) REFERENCES users (id)
);
