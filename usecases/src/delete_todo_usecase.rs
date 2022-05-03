use crate::TodoDeletable;
use entities::Todo;
use uuid::Uuid;

pub struct DeleteTodoUsecase<'a, T> {
	repository: &'a T,
}

impl<T> DeleteTodoUsecase<'_, T>
where
	T: TodoDeletable,
{
	pub fn new(repository: &T) -> DeleteTodoUsecase<T> {
		DeleteTodoUsecase { repository }
	}

	pub fn execute(&self, todo_id: Uuid) {
		self.repository.delete(todo_id);
	}
}
