use std::marker::PhantomData;

use crate::prelude::dice::*;

/// The value generated by `fn_builder`.
pub struct FnBuilder<I, O, IC, OD> {
    input_codie: IC,
    output_die: OD,
    prng: Prng,
    limit: Limit,
    _i: PhantomData<I>,
    _o: PhantomData<O>,
}

impl<I, O, IC, OD> FnBuilder<I, O, IC, OD>
where
    IC: Codie<I>,
    OD: DieOnce<O>,
{
    pub fn build_fn_once(self) -> impl FnOnce(I) -> O {
        move |input| {
            let randomness = self.input_codie.coroll(input);
            let mut prng = self.prng;
            prng.reseed(randomness);
            let mut fate = Fate::new(&mut prng, self.limit);
            self.output_die.roll_once(&mut fate)
        }
    }
}

impl<I, O, IC, OD> FnBuilder<I, O, IC, OD>
where
    IC: Codie<I>,
    OD: Die<O>,
{
    pub fn build_fn(self) -> impl Fn(I) -> O {
        move |input| {
            let randomness = self.input_codie.coroll(input);
            let mut prng = self.prng.clone();
            prng.reseed(randomness);
            let mut fate = Fate::new(&mut prng, self.limit);
            self.output_die.roll(&mut fate)
        }
    }

    pub fn build_fn_mut(mut self) -> impl FnMut(I) -> O {
        move |input| {
            let randomness = self.input_codie.coroll(input);
            let prng = &mut self.prng;
            prng.reseed(randomness);
            let mut fate = Fate::new(prng, self.limit);
            self.output_die.roll(&mut fate)
        }
    }
}

/// Generator for random functions. The generator returns a builder that can be converted into a
/// `Fn`, FnMut` or `FnOnce` implementation.
///
/// This generator does not directly generate an implementation of `Fn`, FnMut` or `FnOnce` because
/// implementing these traits is not stable yet.
pub fn fn_builder<I, O, IC, OD>(
    input_codie: IC,
    output_die: OD,
) -> impl DieOnce<FnBuilder<I, O, IC, OD>>
where
    IC: Codie<I>,
    OD: DieOnce<O>,
{
    dice::from_fn_once(|fate| FnBuilder {
        input_codie,
        output_die,
        prng: fate.prng.fork(),
        limit: fate.limit(),
        _i: PhantomData,
        _o: PhantomData,
    })
}
