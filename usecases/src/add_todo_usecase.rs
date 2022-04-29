use crate::TodoSavable;
use entities::Todo;

pub struct AddTodoUsecase<'a, T> {
	repository: &'a T
}

impl <T> AddTodoUsecase<'_, T> where T: TodoSavable {

	pub fn new(repository: &T) -> AddTodoUsecase<T> {
		AddTodoUsecase{repository}
	}

	pub fn execute(&self, name: String) {
		self.repository.save(Todo{});
	}
}