use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::app::domain::entities::Todo;
use crate::app::domain::repository::{RepositoryError, RepositoryInitError};
use crate::app::domain::usecases::Presenter;

pub struct View {}

impl Presenter for View {
    fn success(&self, result: Vec<Todo>, last_error: Option<RepositoryError>) {
        for todo in result {
            println!("{}", todo);
        }

        if let Some(last_error) = last_error {
            println!("{}", last_error);
        }
    }

    fn failed(&self, error: RepositoryInitError) {
        println!("{}", error);
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.task)
    }
}

impl Display for RepositoryInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", "failed")
    }
}
