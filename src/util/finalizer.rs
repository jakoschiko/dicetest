/// Stores a function that will be executed once the finalizer has been dropped.
///
/// The default use case is to guarantee the execution of cleanup code.
pub struct Finalizer<F>
where
    F: FnOnce(),
{
    f: Option<F>,
}

impl<F> Finalizer<F>
where
    F: FnOnce(),
{
    pub fn new(f: F) -> Self {
        let f = Some(f);
        Finalizer { f }
    }
}

impl<F> Drop for Finalizer<F>
where
    F: FnOnce(),
{
    fn drop(&mut self) {
        let f = self.f.take().unwrap();
        f()
    }
}
