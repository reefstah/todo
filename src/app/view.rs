use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;


use crate::app::domain::entities::Todo;
use crate::app::domain::repository::RepositoryError;
use crate::app::domain::usecases::{Presenter, TodoError};

pub struct View {}
struct List(Vec<Todo>);


impl Presenter for View {
    fn success(&self, result: Vec<Todo>, last_error: Option<RepositoryError>) {
        let list= List(result);
        println!("{}", list);

        if let Some(last_error) = last_error {
            println!("{}", last_error);
        }
    }

    fn failed(&self, error: TodoError) {
        println!("{:?}", error);
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { 
        let vec = &self.0;

        writeln!(f, "TODO\tPRIORITY")?;
        writeln!(f, "----\t--------")?;
        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for v in vec {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            // if count != 0 { write!(f, ", ")?; }
            writeln!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        Ok(())

    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { 
        write!(f, "{0}\t{1}", self.task,self.priority)
    }
}

impl Display for TodoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", "failed")
    }
}
