use std::sync::Arc;

use crate::gen::{Dice, GenOnce, Gen};

/// Adapter for `Gen::arc`.
#[derive(Clone)]
pub struct ArcGen<T> {
    r#dyn: Arc<dyn Gen<T>>,
}

impl<T> ArcGen<T> {
    pub fn new<G>(gen: G) -> Self
    where
        G: Gen<T> + 'static,
    {
        let r#dyn = Arc::new(gen);
        ArcGen { r#dyn }
    }
}

impl<T> Gen<T> for ArcGen<T> {
    fn gen(&self, dice: &mut Dice) -> T {
        self.r#dyn.gen(dice)
    }
}

impl<T> GenOnce<T> for ArcGen<T> {
    fn gen_once(self, dice: &mut Dice) -> T {
        self.gen(dice)
    }
}
