use chrono::prelude::*;
use uuid::Uuid;

#[derive(Clone, Copy, Debug)]
pub struct TodoId(Uuid);

impl From<TodoId> for Uuid {
    fn from(todo_id: TodoId) -> Self {
        todo_id.0
    }
}

impl From<Uuid> for TodoId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl PartialEq for TodoId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EventId(Uuid);

impl From<EventId> for Uuid {
    fn from(event_id: EventId) -> Self {
        event_id.0
    }
}

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: TodoId,
    pub task: String,
    pub calender_date: Option<DateTime<Utc>>,
    pub priority: i8,
}

#[derive(Debug)]
pub enum Event {
    TodoAddedEvent(TodoAddedEvent),
    TodoTagged(TodoId)
}

impl Event {
    pub fn todo_id(&self) -> TodoId {
        match &self {
            Event::TodoAddedEvent(TodoAddedEvent {
                todo
            }) => todo.id,
            Event::TodoTagged(todo_id) => todo_id.to_owned()
        }
    }
}

#[derive(Debug)]
pub struct TodoAddedEvent {
    pub todo: Todo,
}

pub struct TodoDeletedEvent {
    pub id: Uuid,
}
