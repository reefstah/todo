use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use uuid::Uuid;

use crate::app::domain::entities::Event;
use crate::app::domain::entities::Todo;
use crate::app::domain::entities::TodoAddedEvent;
use crate::app::domain::entities::TodoDeletedEvent;
use crate::app::domain::repository::{RepositoryError, RepositoryInitError, Savable};

use super::domain::repository::Deletable;
use super::domain::repository::Repository;
use super::domain::repository::Retrievable;

pub struct SingleFileRepository {}

impl Repository for SingleFileRepository {
    fn retrievable(&self) -> Option<Box<dyn Retrievable>> {
        Some(Box::new(Self{}))
    }

    fn savable(&self) -> Option<Box<dyn Savable>> {
        Some(Box::new(Self{}))
    }

    fn deletable(&self) -> Option<Box<dyn Deletable>> {
        Some(Box::new(Self{}))
    }
}

impl Retrievable for SingleFileRepository {
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

impl Savable for SingleFileRepository {
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

impl Deletable for SingleFileRepository {
    fn delete(&self, id: &str) -> Result<(), RepositoryError> {
        match csv::Reader::from_path("todo.csv") {
            Ok(csv) => {
                let mut count = false;
                let mut iter = csv.into_records();
                let mut temp_file = csv::Writer::from_path(".todo.csv").unwrap();
                temp_file
                    .write_record(&["id", "task", "calender_date", "priority"])
                    .or_else(|_| return Err(RepositoryError::FailedToExecuteDeletedEvent))?;
                while let Some(result) = iter.next() {
                    let field = result.map_err(|_| RepositoryError::FailedToExecuteDeletedEvent)?;
                    if !field.get(0).unwrap().starts_with(id) {
                        temp_file.write_record(&field);
                    } else if !count {
                        count = true;
                    } else {
                        panic!("Id not unique");
                    }
                }
                if !count{
                    println!("No todos found to delete");
                }
                temp_file.flush();
                std::fs::rename(".todo.csv", "todo.csv");
                Ok(())
            }
            Err(_) => Err(RepositoryError::FailedToExecuteDeletedEvent),
        }
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
