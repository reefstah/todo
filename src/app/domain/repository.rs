use std::{error::Error, pin::Pin};
use std::fmt;

use crate::app::domain::entities::Event;

use super::entities::{TodoAddedEvent, TodoDeletedEvent, TodoId, Stream};

pub trait Repository {

    fn retrievable(&self) -> Option<Box<dyn Retrievable>> {
        Option::None
    }

    fn savable(&self) -> Option<Box<dyn Savable>> {
        Option::None
    }

    fn deletable(&self) -> Option<Box<dyn Deletable>> {
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

pub trait RetrievableBreak {
    fn get_events_for(&self, todo_id: TodoId, event_id_offset: u64) -> Box<dyn Stream<Item = Box<dyn Event>> + Unpin>;
}

pub trait Identifyable {
    fn get_todo_ids(&self, event_id_offset: u64) -> Box<dyn Stream<Item = TodoId> + Unpin>;

}

pub trait Savable {
    fn save(&self, event: TodoAddedEvent) -> Result<(), RepositoryError>;
}

pub trait Deletable {
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
