use crate::gen::{Fate, Gen, GenOnce};

/// Adapter for `Gen::boxed`.
pub struct BoxedGen<'a, T> {
    r#dyn: Box<dyn Gen<T> + 'a>,
}

impl<'a, T> BoxedGen<'a, T> {
    pub fn new<G>(gen: G) -> Self
    where
        G: Gen<T> + 'a,
    {
        let r#dyn = Box::new(gen);
        BoxedGen { r#dyn }
    }
}

impl<'a, T> Gen<T> for BoxedGen<'a, T> {
    fn gen(&self, fate: &mut Fate) -> T {
        self.r#dyn.gen(fate)
    }
}

impl<'a, T> GenOnce<T> for BoxedGen<'a, T> {
    fn gen_once(self, fate: &mut Fate) -> T {
        self.gen(fate)
    }
}
