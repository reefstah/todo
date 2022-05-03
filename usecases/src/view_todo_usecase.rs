use crate::TodoViewable;
use entities::Todo;
use uuid::Uuid;

pub struct ViewTodoUsecase<'a, T> {
	repository: &'a T,
}

impl<T> ViewTodoUsecase<'_, T>
where
	T: TodoViewable,
{
	pub fn new(repository: &T) -> ViewTodoUsecase<T> {
		ViewTodoUsecase { repository }
	}

	pub fn execute(&self) {
		self.repository.view();
	}
}
