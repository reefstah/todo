use std::fmt;

use entities::Todo;

pub trait TodoSavable {
    fn save(&self, todo: Todo) -> Result<(), TodoSavableError>;
}

#[derive(Debug)]
pub enum TodoSavableError {
    Failed { source: Box<dyn std::error::Error> },
}

impl fmt::Display for TodoSavableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TodoSavableError::Failed { source } => source.fmt(f),
        }
    }
}

impl std::error::Error for TodoSavableError {}
