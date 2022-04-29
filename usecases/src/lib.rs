mod todo_savable;
mod add_todo_usecase;

pub use todo_savable::TodoSavable;
pub use add_todo_usecase::AddTodoUsecase;

use entities::Todo;

pub fn add_one(x: i32) -> i32 {
    x + 1
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
