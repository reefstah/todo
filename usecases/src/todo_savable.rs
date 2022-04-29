use entities::Todo;

pub trait TodoSavable {
	fn save(&self, todo: Todo);
}