use crate::gen::{Fate, Gen, GenOnce};

/// Adapter for `Gen::boxed`.
pub struct BoxedGen<'a, T> {
    gen: Box<dyn Gen<T> + 'a>,
}

impl<'a, T> BoxedGen<'a, T> {
    pub fn new<G>(gen: G) -> Self
    where
        G: Gen<T> + 'a,
    {
        let gen = Box::new(gen);
        BoxedGen { gen }
    }
}

impl<'a, T> Gen<T> for BoxedGen<'a, T> {
    fn gen(&self, fate: &mut Fate) -> T {
        self.gen.gen(fate)
    }
}

impl<'a, T> GenOnce<T> for BoxedGen<'a, T> {
    fn gen_once(self, fate: &mut Fate) -> T {
        self.gen(fate)
    }
}
