use std::fmt::Display;

use sea_orm::DbErr;

#[derive(Debug)]
pub enum ModelError {
    DatabaseError(DbErr),
    LettreError(lettre::error::Error),
}

impl From<DbErr> for ModelError {
    fn from(err: DbErr) -> Self {
        ModelError::DatabaseError(err)
    }
}

impl From<lettre::error::Error> for ModelError {
    fn from(err: lettre::error::Error) -> Self {
        ModelError::LettreError(err)
    }
}

impl Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelError::DatabaseError(err) => write!(f, "Database error: {}", err),
            ModelError::LettreError(err) => write!(f, "Lettre error: {}", err),
        }
    }
}
