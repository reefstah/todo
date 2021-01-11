use std::error::Error;

use crate::app::domain::entities::Todo;

pub trait Presenter {
    fn success(&self, todos: Vec<Todo>, last_errors: Vec<Box<dyn Error>>);
    fn failed(&self, error: Box<dyn Error>);
}
