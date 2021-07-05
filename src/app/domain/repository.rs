use std::error::Error;
use std::fmt;

use crate::app::domain::entities::Event;

use super::entities::{TodoAddedEvent, TodoDeletedEvent};

pub trait Repository {

    fn retrievable(&self) -> Option<Box<dyn Retrievable>> {
        Option::None
    }

    fn savable(&self) -> Option<Box<dyn Savable<TodoAddedEvent>>> {
        Option::None
    }

    fn deletable(&self) -> Option<Box<dyn Deletable<TodoDeletedEvent>>> {
        Option::None
    }
}

pub trait Retrievable {
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

pub trait Deletable<T> {
    fn delete(&self, id:&str) -> Result<(), RepositoryError>;
}


pub enum RepositoryInitError {
    NotInitialized,
}

#[derive(Debug)]
pub enum RepositoryError {
    DuplicateTodo,
    FailedToExecuteDeletedEvent,
    UnableToSave(Box<dyn std::error::Error>),
}

impl Error for RepositoryError {}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
