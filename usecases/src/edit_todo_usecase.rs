use crate::TodoEditable;
use entities::Todo;
use uuid::Uuid;

pub struct EditTodoUsecase<'a, T> {
	repository: &'a T,
}

impl<T> EditTodoUsecase<'_, T>
where
	T: TodoEditable,
{
	pub fn new(repository: &T) -> EditTodoUsecase<T> {
		EditTodoUsecase { repository }
	}

	pub fn execute(&self, content: String, id: Uuid) {
		let todo = Todo::new(content,id);
		self.repository.edit(todo);
	}
}
