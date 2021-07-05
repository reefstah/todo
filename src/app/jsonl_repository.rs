use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::domain::entities::{Event, Todo, TodoAddedEvent};
use crate::app::domain::repository::{Repository, RepositoryError, RepositoryInitError, Savable};

pub struct JsonlRepository {}

impl Repository for JsonlRepository {
}

impl JsonlRepository {
    pub fn get_events(&self) {
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines("todo.jsonl") {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    println!("{}", ip);
                }
            }
        }
    }
}

impl Savable<TodoAddedEvent> for JsonlRepository {
    fn save(&self, event: TodoAddedEvent) -> Result<(), RepositoryError> {
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
        println!("{:?}",error);
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

#[derive(Serialize, Deserialize)]
enum EventData {
    TodoAddedEvent {
        id: Uuid,
        task: String,
        calender_date: Option<DateTime<Utc>>,
        priority: i8,
    },
}

impl From<TodoAddedEvent> for EventData {
    fn from(event: TodoAddedEvent) -> Self {
        let Todo{id, task, calender_date, priority} = event.todo;
        Self::TodoAddedEvent{id, task, calender_date, priority}
    }
}
