use crate::app::domain::aggregate::get_todos;
use crate::app::domain::entities::{Todo, TodoAddedEvent};
use crate::app::domain::repository::{RepositoryError, RepositoryInitError, Savable};

use super::repository::Retrievable;

pub trait Presenter {
    fn success(&self, result: Vec<Todo>, last_error: Option<RepositoryError>);
    fn failed(&self, error: RepositoryInitError);
}

pub fn show_relevant_usecase<P: Presenter>(repository: &dyn Retrievable, presenter: P) {
    match get_todos(repository) {
        Ok(mut iter) => {
            let mut result = Vec::with_capacity(10);
            let mut last_error: Option<RepositoryError> = None;

            while result.len() < 10 {
                match iter.next() {
                    Some(Ok(todo)) => result.push(todo),
                    Some(Err(_)) => {
                        last_error = Some(RepositoryError::DuplicateTodo);
                    }
                    None => break,
                }
            }
            presenter.success(result, last_error)
        }
        Err(_) => presenter.failed(RepositoryInitError::NotInitialized),
    }
}

pub fn new_todo<T: Savable<TodoAddedEvent>>(repository: T, todo: Todo) -> Result<(), RepositoryError> {
    repository.save(TodoAddedEvent { todo })
}
