use crate::app::domain::aggregate::TodosAggregate;
use crate::app::domain::entities::{Todo, TodoAddedEvent};
use crate::app::domain::repository::{RepositoryError, Savable};

pub use super::aggregate::TodoError;
use super::repository::{Identifyable, Initializable, Retrievable};

pub trait ShowRelevantPresenter {
    fn success(&self, result: Vec<Todo>, last_error: Option<RepositoryError>);
    fn failed(&self, error: TodoError);
}

pub trait NewTodoPresenter {
    fn success(&self);
    fn failed(&self, error: TodoError);
}

pub struct NewTodoUseCase<Repository> {
    repository: Repository,
}

impl<Repository: Savable<TodoAddedEvent>> NewTodoUseCase<Repository> {
    pub fn execute(&self, todo: Todo, presenter: impl NewTodoPresenter) {
        match self.repository.save(TodoAddedEvent { todo }) {
            Ok(_) => presenter.success(),
            Err(e) => presenter.failed(TodoError::Loading(e)),
        }
    }

    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }
}

pub struct ShowRelevantUseCase<'a, Repository: Identifyable + Retrievable> {
    aggregate: TodosAggregate<'a, Repository>,
}

impl<'a, Repository: Identifyable + Retrievable> ShowRelevantUseCase<'a, Repository> {
    pub fn execute(&'a self, presenter: impl ShowRelevantPresenter) {
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

pub trait InitializePresenter {
    fn success(&self);
    fn failed(&self, error: TodoError);
}

pub struct InitializeUseCase<Repository> {
    repository: Repository,
}

impl<Repository: Initializable> InitializeUseCase<Repository> {
    pub fn execute(&self, presenter: impl InitializePresenter) {
        let result = self.repository.initialize();

        match result {
            Ok(_) => presenter.success(),
            Err(e) => presenter.failed(TodoError::Loading(e)),
        }
    }

    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }
}
