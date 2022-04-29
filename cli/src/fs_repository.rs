use usecases::TodoSavable;
use entities::Todo;

pub struct FileSystemRepository {}

impl TodoSavable for FileSystemRepository {
	
fn save(&self, todo: Todo) { 
	fs_repository::save(todo);
}
}