use uuid::Uuid;

pub trait TodoInteractiveEditable {
	fn edit(&self, todo_id: Uuid);
}