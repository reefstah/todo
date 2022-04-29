use usecases::AddTodoPresenter;
use uuid::Uuid;

pub struct AddTodoCliPresenter<'a> {
    todo_id: &'a Uuid,
}

impl<'a> AddTodoCliPresenter<'a> {
    pub fn new(todo_id: &'a Uuid) -> AddTodoCliPresenter<'a> {
        Self { todo_id }
    }
}

impl AddTodoPresenter for AddTodoCliPresenter<'_> {
    fn failed(&self, _error: impl std::error::Error) {
        println!(
            "Something went wrong while saving todo with id {}",
            self.todo_id
        );
    }

    fn success(&self) {
        println!("Successfully added todo with id {}", self.todo_id);
    }
}
