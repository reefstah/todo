use chrono::prelude::*;
use uuid::Uuid;

use crate::app::domain::repository::RepositoryError;

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: Uuid,
    pub task: String,
    pub calender_date: Option<DateTime<Utc>>,
    pub priority: i8,
}

pub trait Event {
    fn handle(&self, todo: Option<Todo>) -> Result<Todo, RepositoryError>;
}

pub struct TodoAddedEvent {
    pub todo: Todo,
}

pub struct TodoDeletedEvent {
    pub id: Uuid,
}

//pub struct TodoChangedEvent {
//}

impl Event for TodoAddedEvent {
    fn handle(&self, todo: Option<Todo>) -> Result<Todo, RepositoryError> {
        if let Some(_) = todo {
            return Err(RepositoryError::DuplicateTodo);
        }

        Ok(self.todo.clone())
    }
}

//struct TaskChangedEvent {
//    pub value: String,
//}
//
//struct OrderedEvent {
//    seq: u64,
//    creation_date: DateTime<Utc>,
//    event: dyn Event,
//}
