use ::rng::Rng;
use ::gen::{Size, GenOnce, Wrapper, GenOnceWrapper};

/// Adapter for `GenOnce::boxed`.
pub struct GenBoxedOnce<T> {
    boxed: Box<dyn Wrapper<T>>,
}

impl<T> GenBoxedOnce<T> {
    pub fn new<G>(gen: G) -> Self
    where
        T: 'static,
        G: GenOnce<T> + 'static,
    {
        let wrapper = GenOnceWrapper::new(gen);
        let boxed = Box::new(wrapper);
        GenBoxedOnce { boxed }
    }

    pub fn from_boxed(boxed: Box<dyn Wrapper<T>>) -> Self {
        GenBoxedOnce { boxed }
    }
}

impl<T> GenOnce<T> for GenBoxedOnce<T> {
    fn gen_once(mut self, rng: &mut Rng, size: Size) -> T {
        self.boxed.gen_once(rng, size)
    }


    fn boxed_once(self) -> GenBoxedOnce<T>
    where
        Self: Sized + 'static,
        T: 'static,
    {
        self
    }
}
