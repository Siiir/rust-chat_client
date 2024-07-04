pub struct Defer<F: FnOnce()>(Option<F>);

impl<F> Drop for Defer<F>
where
    F: FnOnce(),
{
    fn drop(&mut self) {
        self.0.take().map(|f| f());
    }
}
