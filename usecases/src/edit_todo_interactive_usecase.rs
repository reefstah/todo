use crate::TodoInteractiveEditable;
use entities::Todo;
use uuid::Uuid;

pub struct EditTodoInteractiveUsecase<'a, T> {
	repository: &'a T,
}

impl<T> EditTodoInteractiveUsecase<'_, T>
where
	T: TodoInteractiveEditable,
{
	pub fn new(repository: &T) -> EditTodoInteractiveUsecase<T> {
		EditTodoInteractiveUsecase { repository }
	}

	pub fn execute(&self, todo_id: Uuid) {
		self.repository.edit(todo_id);
	}
}
