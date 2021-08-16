use std::pin::Pin;
use std::sync::Arc;

use uuid::Uuid;

use crate::app::domain::entities::{Todo, Map};
use crate::app::domain::repository::{RepositoryError, RepositoryInitError};

use super::entities::{Event, Stream};
use super::repository::{Identifyable, RetrievableBreak};

pub struct TodoAggregate<'a, Repository: Identifyable + RetrievableBreak> {
    repository: &'a Repository
}


impl <'a, Repository: Identifyable + RetrievableBreak> TodoAggregate<'a, Repository> {
    pub fn get_todos(&self) -> Box<dyn Stream<Item = Todo> + Unpin + '_> {

        self.repository.get_todo_ids(0)
            .map(Box::new(move |id| {
               //let stat = self.repository.get_events_for(id, 0);
                //let mut events: Vec<Box<dyn Event>> = self.repository.get_events_for(id, 0).into();
                let events: Vec<Box<dyn Event>> = self.repository.get_events_for(id,0).into();

                for event in events {
                }

                Uuid::new_v4()
            }))
            .map(Box::new(|id| Todo{
                id,
                task: String::from(""),
                priority: 3,
                calender_date: Option::None
            }))
    }

    pub fn new(repository: &'a Repository) -> Self {
        TodoAggregate{repository}
    }
}
