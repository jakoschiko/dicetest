use std::rc::Rc;

use ::rng::Rng;
use ::gen::{Params, GenOnce, Gen, Wrapper, GenWrapper};

#[derive(Clone)]
pub struct GenRc<T> {
    rc: Rc<dyn Wrapper<T>>,
}

impl<T> GenRc<T> {
    pub fn new<G>(gen: G) -> Self
    where
        T: 'static,
        G: Gen<T> + 'static,
    {
        let wrapper = GenWrapper::new(gen);
        let rc: Rc<dyn Wrapper<T>> = Rc::new(wrapper);
        GenRc { rc }
    }

    pub fn from_boxed(boxed: Box<dyn Wrapper<T>>) -> Self {
        let rc = boxed.into();
        GenRc { rc }
    }
}

impl<T> Gen<T> for GenRc<T> {
    fn gen(&self, rng: &mut Rng, params: &Params) -> T {
        self.rc.gen(rng, params)
    }

    fn rc(self) -> GenRc<T>
        where
        Self: Sized + 'static,
        T: 'static,
    {
        self
    }
}

impl<T> GenOnce<T> for GenRc<T> {
    fn gen_once(self, rng: &mut Rng, params: &Params) -> T {
        self.gen(rng, params)
    }
}
