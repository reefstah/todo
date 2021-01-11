use crate::app::domain::entities::Todo;
use crate::app::domain::repository::{Repository, RepositoryError, RepositoryInitError};

pub fn get_todos<T: Repository>(
    repository: T,
) -> Result<Box<dyn Iterator<Item = Result<Todo, RepositoryError>>>, RepositoryInitError> {
    match repository.get_events(0) {
        Ok(iter) => {
            Ok(Box::new( iter.flat_map(|result| result.map(|event| event.handle(None)))))
        }
        Err(e) => Err(e),
    }
}
