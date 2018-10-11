use ::rng::Rng;
use ::gen::{Limit, GenOnce, Gen, Wrapper, GenWrapper, adapters::{GenBoxedOnce, GenRc, GenArc}};

/// Adapter for `Gen::boxed`.
pub struct GenBoxed<T> {
    boxed: Box<dyn Wrapper<T>>,
}

impl<T> GenBoxed<T> {
    pub fn new<G>(gen: G) -> Self
    where
        T: 'static,
        G: Gen<T> + 'static,
    {
        let wrapper = GenWrapper::new(gen);
        let boxed = Box::new(wrapper);
        GenBoxed { boxed }
    }
}

impl<T> Gen<T> for GenBoxed<T> {
    fn gen(&self, rng: &mut Rng, lim: Limit) -> T {
        self.boxed.gen(rng, lim)
    }

    fn boxed(self) -> GenBoxed<T>
    where
        Self: Sized + 'static,
        T: 'static,
    {
        self
    }

    fn rc(self) -> GenRc<T>
    where
        Self: Sized + 'static,
        T: 'static,
    {
        GenRc::from_boxed(self.boxed)
    }

    fn arc(self) -> GenArc<T>
    where
        Self: Sized + 'static,
        T: 'static,
    {
        GenArc::from_boxed(self.boxed)
    }
}

impl<T> GenOnce<T> for GenBoxed<T> {
    fn gen_once(self, rng: &mut Rng, lim: Limit) -> T {
        self.gen(rng, lim)
    }

    fn boxed_once(self) -> GenBoxedOnce<T>
    where
        Self: Sized + 'static,
        T: 'static,
    {
        GenBoxedOnce::from_boxed(self.boxed)
    }
}
