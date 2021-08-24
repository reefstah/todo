use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::domain::entities::{Todo, TodoAddedEvent};
use crate::app::domain::repository::{Repository, RepositoryError, Savable};

use super::domain::entities::{Event, TodoId};
use super::domain::repository::{Identifyable, Result, Retrievable, TodoEventIter};

pub struct JsonlRepository {}

impl Repository for JsonlRepository {}

impl Identifyable for JsonlRepository {
    fn get_todo_ids(
        &self,
    ) -> Result<Box<dyn Iterator<Item = Result<super::domain::entities::TodoId>>>> {
        let reader = EventDataReader::new("todo.jsonl")?;
        Ok(Box::new(TodoIdReader::new(reader)))
    }
}

impl Retrievable for JsonlRepository {
    fn get_events(&self, todo_id: TodoId) -> Result<TodoEventIter> {
        let reader = EventDataReader::new("todo.jsonl")?;
        Ok(Box::new(TodoEventReader::new(reader).filter(
            move |event| {
                if let Ok(event) = event {
                    event.todo_id() == todo_id
                } else {
                    false
                }
            },
        )))
    }
}

struct TodoEventReader {
    event_reader: EventDataReader,
}

impl TodoEventReader {
    fn new(event_reader: EventDataReader) -> Self {
        Self { event_reader }
    }
}

impl Iterator for TodoEventReader {
    type Item = Result<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.event_reader.next() {
            Some(Ok(EventData::TodoAddedEvent {
                todo_id,
                event_id: _,
                task,
                calender_date,
                priority,
            })) => Some(Ok(Event::TodoAddedEvent(TodoAddedEvent {
                todo: Todo {
                    id: todo_id.into(),
                    task,
                    calender_date,
                    priority,
                },
            }))),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
}

struct TodoIdReader {
    event_reader: EventDataReader,
}

impl TodoIdReader {
    fn new(event_reader: EventDataReader) -> Self {
        Self { event_reader }
    }
}

impl Iterator for TodoIdReader {
    type Item = Result<TodoId>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.event_reader.next() {
            Some(Ok(EventData::TodoAddedEvent {
                todo_id,
                event_id: _,
                task: _,
                calender_date: _,
                priority: _,
            })) => Some(Ok(todo_id.into())),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
}

struct EventDataReader {
    lines: io::Lines<io::BufReader<File>>,
}

impl EventDataReader {
    fn new(filename: &str) -> Result<Self> {
        read_lines(filename)
            .map(|lines| Self { lines })
            .map_err(|e| RepositoryError::FailedReadingSource(Box::new(e)))
    }
}

impl Iterator for EventDataReader {
    type Item = Result<EventData>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Some(Ok(line)) => {
                let result: Result<EventData> = serde_json::from_str(&line)
                    .map_err(|e| RepositoryError::FailedWhileReadingSource(Box::new(e)));
                Some(result)
            }
            Some(Err(e)) => Some(Err(RepositoryError::FailedWhileReadingSource(Box::new(e)))),
            None => None,
        }
    }
}

impl Savable<TodoAddedEvent> for JsonlRepository {
    fn save(&self, event: TodoAddedEvent) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("todo.jsonl")?;

        let event: EventData = event.into();
        let mut json = serde_json::to_string(&event).unwrap();
        json.push_str("\n");

        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

impl From<std::io::Error> for RepositoryError {
    fn from(error: std::io::Error) -> Self {
        println!("{:?}", error);
        Self::UnableToSave(Box::new(error))
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Serialize, Deserialize)]
enum EventData {
    TodoAddedEvent {
        todo_id: Uuid,
        event_id: Uuid,
        task: String,
        calender_date: Option<DateTime<Utc>>,
        priority: i8,
    },
}

impl From<TodoAddedEvent> for EventData {
    fn from(event: TodoAddedEvent) -> Self {
        let Todo {
            id,
            task,
            calender_date,
            priority,
        } = event.todo;
        Self::TodoAddedEvent {
            event_id: id.into(),
            todo_id: id.into(),
            task,
            calender_date,
            priority,
        }
    }
}
