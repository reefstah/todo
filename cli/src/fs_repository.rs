use entities::Todo;
use usecases::TodoEditable;
use usecases::TodoSavable;
use usecases::TodoInteractiveEditable;
use uuid::Uuid;

pub struct FileSystemRepository {}

impl TodoSavable for FileSystemRepository {
	fn save(&self, todo: Todo) {
		fs_repository::add(todo);
	}
}

impl TodoEditable for FileSystemRepository {
	fn edit(&self, todo: Todo) {
		fs_repository::edit(todo);
	}
}

impl TodoInteractiveEditable for FileSystemRepository {
	fn edit(&self, todo_id: Uuid) {
		fs_repository::edit_iteractive(todo_id);
	}
}
