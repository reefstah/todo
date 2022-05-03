use entities::Todo;
use usecases::TodoEditable;
use usecases::TodoInteractiveEditable;
use usecases::TodoSavable;
use usecases::TodoViewable;
use usecases::TodoSavableError;
use usecases::TodoDeletable;
use uuid::Uuid;

pub struct FileSystemRepository {}

impl TodoSavable for FileSystemRepository {
    fn save(&self, todo: Todo) -> Result<(), TodoSavableError> {
        fs_repository::add(todo).map_err(|op| TodoSavableError::Failed {
            source: Box::new(op),
        })
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


impl TodoViewable for FileSystemRepository {
    fn view(&self) {
        fs_repository::view();
    }
}


impl TodoDeletable for FileSystemRepository {
    fn delete(&self, todo_id: Uuid) {
        fs_repository::delete(todo_id);
    }
}