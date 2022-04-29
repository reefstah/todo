pub trait AddTodoPresenter {
    fn success(&self);
    fn failed(&self, error: impl std::error::Error);
}
