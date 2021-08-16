use std::task::Wake;

pub struct Waker {

}

impl Wake for Waker {
    fn wake(self: std::sync::Arc<Self>) {
        self.
    }
}
