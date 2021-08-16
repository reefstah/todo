
use crate::app::domain::entities::{Take, Todo, TodoAddedEvent};
use crate::app::domain::repository::{RepositoryError, RepositoryInitError, Savable};

use super::aggregate::TodoAggregate;
use super::repository::{Identifyable, RetrievableBreak};

pub trait Presenter {
    fn success(&self, result: Vec<Todo>, last_error: Option<RepositoryError>);
    fn failed(&self, error: RepositoryInitError);
}

pub struct ShowRelevantUseCase<'a, Repository: Identifyable + RetrievableBreak> {
    aggregate: TodoAggregate<'a, Repository>,
}

impl<'a, Repository: Identifyable + RetrievableBreak> ShowRelevantUseCase<'a, Repository> {
    pub fn execute(&'a self, presenter: impl Presenter) {
        let mut todos = self.aggregate.get_todos();
        let last_error: Option<RepositoryError> = None;

        let result = todos.take(10);

        presenter.success(result, last_error)
    }

    pub fn new(repository: &'a Repository) -> Self {
        Self{aggregate: TodoAggregate::new(repository)}
    }
}

pub fn new_todo<T: Savable>(repository: T, todo: Todo) -> Result<(), RepositoryError> {
    repository.save(TodoAddedEvent { todo })
}
