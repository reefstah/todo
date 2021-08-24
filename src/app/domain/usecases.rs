use crate::app::domain::aggregate::TodosAggregate;
use crate::app::domain::entities::{Todo, TodoAddedEvent};
use crate::app::domain::repository::{RepositoryError, Savable};

pub use super::aggregate::TodoError;
use super::repository::{Identifyable, Retrievable};

pub trait Presenter {
    fn success(&self, result: Vec<Todo>, last_error: Option<RepositoryError>);
    fn failed(&self, error: TodoError);
}

pub fn new_todo<T: Savable<TodoAddedEvent>>(
    repository: T,
    todo: Todo,
) -> Result<(), RepositoryError> {
    repository.save(TodoAddedEvent { todo })
}

impl<'a, Repository: Identifyable + Retrievable> ShowRelevantUseCase<'a, Repository> {
    pub fn execute(&'a self, presenter: impl Presenter) {
        let todos = self.aggregate.get_todos();
        let last_error: Option<RepositoryError> = None;

        let result = todos.unwrap().take(10).map(|todo| todo.unwrap()).collect();

        presenter.success(result, last_error)
    }

    pub fn new(repository: &'a Repository) -> Self {
        Self {
            aggregate: TodosAggregate::new(repository),
        }
    }
}

pub struct ShowRelevantUseCase<'a, Repository: Identifyable + Retrievable> {
    aggregate: TodosAggregate<'a, Repository>,
}
