use std::{
    borrow::{Borrow, BorrowMut},
    marker::PhantomData,
    ops::DerefMut,
    pin::Pin,
    sync::Arc,
    task::Waker,
    task::{Context, Poll, Wake},
    thread::{self, Thread},
};

use chrono::prelude::*;
use uuid::Uuid;

use crate::app::domain::repository::RepositoryError;

pub type TodoId = Uuid;

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: TodoId,
    pub task: String,
    pub calender_date: Option<DateTime<Utc>>,
    pub priority: i8,
}

impl Todo {
    pub fn from(event: TodoAddedEvent) {}

    pub fn update(event: impl Event) {}
}

pub trait Event {
    fn handle(&self, todo: Option<Todo>) -> Result<Todo, RepositoryError>;
}

pub struct TodoAddedEvent {
    pub todo: Todo,
}

pub struct TodoTaggedEvent {}

pub struct TodoDeletedEvent {
    pub id: Uuid,
}

pub trait Stream {
    type Item;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}

impl<S: ?Sized + Stream + Unpin> Stream for Box<S> {
    type Item = S::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut **self).poll_next(cx)
    }
}

impl<S: ?Sized + Stream + Unpin> Stream for &mut S {
    type Item = S::Item;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        S::poll_next(Pin::new(&mut **self), cx)
    }
}

pub trait Map<'a, Input> {
    fn map<Output: 'a + Unpin>(
        self,
        apply: Box<dyn FnMut(Input) -> Output + 'a>,
    ) -> Box<dyn Stream<Item = Output> + 'a + Unpin>
    where
        Input: 'a + Unpin;
}

impl<'a, Input, S: 'a + ?Sized + Stream<Item = Input> + Unpin> Map<'a, Input> for Box<S> {
    fn map<Output: 'a + Unpin>(
        self,
        apply: Box<dyn FnMut(Input) -> Output + 'a>,
    ) -> Box<dyn Stream<Item = Output> + 'a + Unpin>
    where
        Input: 'a + Unpin,
    {
        Box::new(MapImpl {
            stream: self,
            apply: Box::new(apply),
        })
    }
}

pub struct MapImpl<StreamImpl: ?Sized, Apply> {
    stream: Box<StreamImpl>,
    apply: Box<Apply>,
}

impl<Apply, StreamImpl: ?Sized + Unpin + Stream<Item = Input>, Input: Unpin, Output: Unpin> Stream
    for MapImpl<StreamImpl, Apply>
where
    Apply: FnMut(Input) -> Output,
{
    type Item = Output;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let pin = Pin::new(self.stream.as_mut());
        match pin.poll_next(cx) {
            Poll::Ready(Some(result)) => Poll::Ready(Some((self.apply)(result))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

pub trait Take<T> {
    fn take(self: &mut Self, count: usize) -> Vec<T>;
}

impl<S: ?Sized + Stream<Item = T> + Unpin, T: Unpin> Take<T> for Box<S> {
    fn take(mut self: &mut Self, count: usize) -> Vec<T> {
        let t = thread::current();
        let waker = Arc::new(ThreadWaker(t)).into();
        let mut cx = Context::from_waker(&waker);
        let mut result = Vec::with_capacity(count);

        while result.len() < count {
            match Pin::new(&mut self).poll_next(&mut cx) {
                Poll::Ready(Some(res)) => result.push(res),
                Poll::Ready(None) => break,
                Poll::Pending => thread::park(),
            }
        }
        result
    }
}

impl<T> From<Box<dyn Stream<Item = T> + Unpin>> for Vec<T> {
    fn from(mut stream: Box<dyn Stream<Item = T> + Unpin>) -> Self {
        
        let t = thread::current();
        let waker = Arc::new(ThreadWaker(t)).into();
        let mut cx = Context::from_waker(&waker);
        let mut result = Vec::new();

        loop {
            match Pin::new(&mut stream).poll_next(&mut cx) {
                Poll::Ready(Some(res)) => result.push(res),
                Poll::Ready(None) => break,
                Poll::Pending => thread::park(),
            }
        }
        result
    }
}

//pub struct TodoChangedEvent {
//}

impl Event for TodoAddedEvent {
    fn handle(&self, todo: Option<Todo>) -> Result<Todo, RepositoryError> {
        if let Some(_) = todo {
            return Err(RepositoryError::DuplicateTodo);
        }

        Ok(self.todo.clone())
    }
}

//struct TaskChangedEvent {
//    pub value: String,
//}
//
//struct OrderedEvent {
//    seq: u64,
//    creation_date: DateTime<Utc>,
//    event: dyn Event,
//}
