use chrono::NaiveDateTime;

use crate::models::movie::Movie;

pub struct MovieBuilder {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub imdb_id: Option<String>,
    pub year: Option<u16>,
    pub critic_score: Option<String>,
    pub genre: Option<String>,
    pub director: Option<String>,
    pub actors: Option<String>,
    pub plot: Option<String>,
    pub poster: Option<String>,
    pub is_good: Option<bool>,
    pub last_watched: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl MovieBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            actors: None,
            created_at: None,
            critic_score: None,
            deleted_at: None,
            director: None,
            genre: None,
            imdb_id: None,
            is_good: None,
            last_watched: None,
            plot: None,
            poster: None,
            title: None,
            updated_at: None,
            year: None,
        }
    }

    pub fn actors(mut self, actors: impl Into<String>) -> Self {
        self.actors = Some(actors.into());
        self
    }

    pub fn created_at(mut self, created_at: NaiveDateTime) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn critic_score(mut self, critic_score: impl Into<String>) -> Self {
        self.critic_score = Some(critic_score.into());
        self
    }

    // pub fn deleted_at(mut self, deleted_at: NaiveDateTime) -> Self {
    //     self.deleted_at = Some(deleted_at);
    //     self
    // }

    pub fn director(mut self, director: impl Into<String>) -> Self {
        self.director = Some(director.into());
        self
    }

    pub fn imdb_id(mut self, imdb_id: impl Into<String>) -> Self {
        self.imdb_id = Some(imdb_id.into());
        self
    }

    // pub fn is_good(mut self, is_good: impl Into<bool>) -> Self {
    //     self.is_good = Some(is_good.into());
    //     self
    // }

    pub fn year(mut self, year: impl Into<u16>) -> Self {
        self.year = Some(year.into());
        self
    }

    pub fn genre(mut self, genre: impl Into<String>) -> Self {
        self.genre = Some(genre.into());
        self
    }

    // pub fn last_watched(mut self, last_watched: Option<NaiveDateTime>) -> Self {
    //     self.last_watched = last_watched.or(None);
    //     self
    // }

    pub fn plot(mut self, plot: impl Into<String>) -> Self {
        self.plot = Some(plot.into());
        self
    }

    pub fn poster(mut self, poster: impl Into<String>) -> Self {
        self.poster = Some(poster.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn updated_at(mut self, updated_at: NaiveDateTime) -> Self {
        self.updated_at = Some(updated_at);
        self
    }

    pub fn build(self) -> Movie {
        Movie {
            id: self.id,
            title: self.title,
            imdb_id: self.imdb_id,
            year: self.year,
            critic_score: self.critic_score,
            genre: self.genre,
            director: self.director,
            actors: self.actors,
            plot: self.plot,
            poster: self.poster,
            is_good: self.is_good,
            last_watched: self.last_watched,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
        }
    }
}