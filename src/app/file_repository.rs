use std::fs::File;
use std::fs::OpenOptions;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::domain::entities::Event;
use crate::app::domain::entities::Todo;
use crate::app::domain::entities::TodoAddedEvent;
//use crate::app::domain::entities::TodoChangedEvent;
use crate::app::domain::repository::{Repository, RepositoryError, RepositoryInitError, Savable};

pub struct SingleFileRepository {}

impl Repository for SingleFileRepository {
    fn get_events(
        &self,
        _seq: u64,
    ) -> Result<
        Box<dyn Iterator<Item = Result<Box<dyn Event>, RepositoryError>>>,
        RepositoryInitError,
    > {
        match csv::Reader::from_path("todo.csv") {
            Ok(csv) => {
                let reader = CSVIter {
                    iter: csv.into_deserialize(),
                };

                Ok(Box::new(reader))
            }
            Err(_) => Err(RepositoryInitError::NotInitialized),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Record {
    id: Uuid,
    task: String,
    calender_date: Option<DateTime<Utc>>,
    priority: i8,
}

impl From<Todo> for Record {
    fn from(todo: Todo) -> Self {
        Record {
            id: todo.id,
            task: todo.task,
            calender_date: todo.calender_date,
            priority: todo.priority,
        }
    }
}

impl From<Record> for Todo {
    fn from(record: Record) -> Self {
        Todo {
            id: record.id,
            task: record.task,
            calender_date: record.calender_date,
            priority: record.priority,
        }
    }
}

impl Savable<TodoAddedEvent> for SingleFileRepository {
    fn save(&self, event: TodoAddedEvent) -> Result<(), RepositoryError> {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("todo.csv")
            .map_err(|err| RepositoryError::UnableToSave(Box::new(err)))?;

        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(file);

        writer
            .serialize(Record::from(event.todo))
            .map_err(|err| RepositoryError::UnableToSave(Box::new(err)))?;

        writer
            .flush()
            .map_err(|err| RepositoryError::UnableToSave(Box::new(err)))?;
        Ok(())
    }
}

//impl Savable<TodoChangedEvent> for SingleFileRepository {
//    fn save(&self, event: TodoChangedEvent) -> Result<(), RepositoryError> {
//        Ok(())
//    }
//}

pub fn init() -> Result<(), RepositoryInitError> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("todo.csv")
        .or_else(|_| return Err(RepositoryInitError::NotInitialized))?;

    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    writer
        .write_record(&["id", "task", "calender_date", "priority"])
        .or_else(|_| return Err(RepositoryInitError::NotInitialized))?;
    writer
        .flush()
        .or_else(|_| return Err(RepositoryInitError::NotInitialized))?;

    Ok(())
}

struct CSVIter {
    iter: csv::DeserializeRecordsIntoIter<File, Record>,
}

impl Iterator for CSVIter {
    type Item = Result<Box<dyn Event>, RepositoryError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(x) => {
                //let line = self.iter.reader().position().line();
                match x {
                    Ok(record) => Some(Ok(Box::new(TodoAddedEvent {
                        todo: record.into(),
                    }))),
                    Err(_) => Some(Err(RepositoryError::DuplicateTodo)),
                }
            }
            None => None,
        }
    }
}
