use std::error::Error;
use std::fmt;

use crate::app::domain::entities::Event;

pub trait Repository {
    fn get_events(
        &self,
        seq: u64,
    ) -> Result<
        Box<dyn Iterator<Item = Result<Box<dyn Event>, RepositoryError>>>,
        RepositoryInitError,
    >;
}

pub trait Savable<T> {
    fn save(&self, event: T) -> Result<(), RepositoryError>;
}

pub enum RepositoryInitError {
    NotInitialized,
}

#[derive(Debug)]
pub enum RepositoryError {
    DuplicateTodo,
    UnableToSave(Box<dyn std::error::Error>),
}

impl Error for RepositoryError {}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
