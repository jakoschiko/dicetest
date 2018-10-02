use std::sync::Arc;

use ::rng::Rng;
use ::gen::{Size, GenOnce, Gen, Wrapper, GenWrapper};

/// Default implementation for `Gen::arc`.
#[derive(Clone)]
pub struct GenArc<T> {
    arc: Arc<dyn Wrapper<T>>,
}

impl<T> GenArc<T> {
    pub fn new<G>(gen: G) -> Self
    where
        T: 'static,
        G: Gen<T> + 'static,
    {
        let wrapper = GenWrapper::new(gen);
        let arc = Arc::new(wrapper);
        GenArc { arc }
    }

    pub fn from_boxed(boxed: Box<dyn Wrapper<T>>) -> Self {
        let arc = boxed.into();
        GenArc { arc }
    }
}

impl<T> Gen<T> for GenArc<T> {
    fn gen(&self, rng: &mut Rng, size: Size) -> T {
        self.arc.gen(rng, size)
    }

    fn arc(self) -> GenArc<T>
        where
        Self: Sized + 'static,
        T: 'static,
    {
        self
    }
}

impl<T> GenOnce<T> for GenArc<T> {
    fn gen_once(self, rng: &mut Rng, size: Size) -> T {
        self.gen(rng, size)
    }
}
