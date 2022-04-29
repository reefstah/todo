use crate::{AddTodoPresenter, TodoSavable};
use entities::Todo;
use uuid::Uuid;

pub struct AddTodoUsecase<'a, T> {
    repository: &'a T,
}

impl<T> AddTodoUsecase<'_, T>
where
    T: TodoSavable,
{
    pub fn new(repository: &T) -> AddTodoUsecase<T> {
        AddTodoUsecase { repository }
    }

    pub fn execute(&self, content: String, id: Uuid, presenter: &impl AddTodoPresenter) {
        let todo = Todo::new(content, id);
        if let Err(error) = self.repository.save(todo) {
            presenter.failed(error)
        } else {
            presenter.success();
        }
    }
}
