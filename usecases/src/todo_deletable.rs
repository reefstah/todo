use uuid::Uuid;

pub trait TodoDeletable {
	fn delete(&self, todo_id: Uuid);
}