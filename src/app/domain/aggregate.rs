use crate::app::domain::entities::{Event, Todo, TodoAddedEvent, TodoId};
use crate::app::domain::repository::RepositoryError;

use super::repository::{Identifyable, Retrievable};

pub struct TodosAggregate<'a, T> {
    repository: &'a T,
}

impl<'a, T: Retrievable + Identifyable> TodosAggregate<'a, T> {
    pub fn get_todos(
        &self,
    ) -> Result<impl Iterator<Item = Result<Todo, TodoError>> + '_, RepositoryError> {
        let repository = &self.repository;
        Ok(repository.get_todo_ids()?.map(move |todo_id| {
            let mut result: Option<Todo> = None;
            let todo_id: TodoId = todo_id?;
            let mut buffer = Vec::new();

            for event in repository.get_events(todo_id)? {
                match event {
                    Ok(Event::TodoAddedEvent(TodoAddedEvent { todo })) => {
                        result = Some(todo.to_owned())
                    }
                    Err(e) => return Err(TodoError::Loading(e)),
                    Ok(event) => {
                        if let None = result {
                            if buffer.len() == 10 {
                                return Err(TodoError::TodoAddedEventNotFound);
                            }
                            buffer.push(event);
                        }
                    }
                }
            }

            match result {
                Some(todo) => Ok(todo),
                None => Err(TodoError::TodoNotFound),
            }
        }))
    }
    pub fn new(repository: &'a T) -> Self {
        Self { repository }
    }
}

#[derive(Debug)]
pub enum TodoError {
    Loading(RepositoryError),
    TodoAddedEventNotFound,
    TodoNotFound,
}

impl From<RepositoryError> for TodoError {
    fn from(e: RepositoryError) -> Self {
        TodoError::Loading(e)
    }
}
