use std::sync::Arc;

use crate::gen::{Fate, Gen, GenOnce};

/// Adapter for `Gen::arc`.
#[derive(Clone)]
pub struct ArcGen<T> {
    gen: Arc<dyn Gen<T>>,
}

impl<T> ArcGen<T> {
    pub fn new<G>(gen: G) -> Self
    where
        G: Gen<T> + 'static,
    {
        let gen = Arc::new(gen);
        ArcGen { gen }
    }
}

impl<T> Gen<T> for ArcGen<T> {
    fn gen(&self, fate: &mut Fate) -> T {
        self.gen.gen(fate)
    }
}

impl<T> GenOnce<T> for ArcGen<T> {
    fn gen_once(self, fate: &mut Fate) -> T {
        self.gen(fate)
    }
}
