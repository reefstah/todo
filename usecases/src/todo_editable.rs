use entities::Todo;

pub trait TodoEditable {
	fn edit(&self, todo: Todo);
}