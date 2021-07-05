use crate::app::domain::entities::Todo;
use crate::app::domain::repository::{RepositoryError, RepositoryInitError};

use super::repository::Retrievable;

pub fn get_todos(
    repository: &dyn Retrievable,
) -> Result<Box<dyn Iterator<Item = Result<Todo, RepositoryError>>>, RepositoryInitError> {
    match repository.get_events(0) {
        Ok(iter) => {
            Ok(Box::new( iter.flat_map(|result| result.map(|event| event.handle(None)))))
        }
        Err(e) => Err(e),
    }
}
