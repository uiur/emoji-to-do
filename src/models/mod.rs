use std::collections::HashMap;

pub mod reaction;
pub mod team;
pub mod user;

#[derive(Debug)]
pub enum Error {
    SqlError(sqlx::Error),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SqlError(e) => e.fmt(f),
        }
    }
}
impl actix_web::error::ResponseError for Error {}
impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::SqlError(e)
    }
}

impl std::error::Error for Error {}
