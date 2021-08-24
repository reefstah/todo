use std::{error::Error, fmt::Display};
use std::{fmt, result};

use super::entities::{Event, TodoAddedEvent, TodoDeletedEvent, TodoId};

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

pub type Result<T> = result::Result<T, RepositoryError>;

pub trait Retrievable {
    fn get_events(&self, todo_id: TodoId) -> Result<TodoEventIter>;
}

pub trait Identifyable {
    fn get_todo_ids(&self) -> Result<TodoIdIter>;
}

pub trait Savable<T> {
    fn save(&self, event: T) -> Result<()>;
}

pub trait Deletable<T> {
    fn delete(&self, id: &str) -> Result<()>;
}

#[derive(Debug)]
pub enum RepositoryError {
    UnableToSave(Box<dyn std::error::Error>),
    FailedReadingSource(Box<dyn std::error::Error>),
    FailedWhileReadingSource(Box<dyn Error>),
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self)
    }
}

pub type TodoEventIter = Box<dyn Iterator<Item = Result<Event>>>;
pub type TodoIdIter = Box<dyn Iterator<Item = Result<TodoId>>>;
