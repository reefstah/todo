use uuid::Uuid;

pub struct Todo {
	content : String,
	todo_id: uuid::Uuid,
}

impl Todo{
	pub fn content(&self) -> &str {
		&self.content
	}
	pub fn id(&self) -> &Uuid {
		&self.todo_id
	}
	pub fn new(content: String, todo_id: Uuid) -> Self{
		Self{
			content,
			todo_id,
		}
	}
}