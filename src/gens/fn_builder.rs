use std::marker::PhantomData;

use crate::prelude::gens::*;

/// The value generated by `fn_builder`.
pub struct FnBuilder<I, O, IC, OG> {
    input_cogen: IC,
    output_gen: OG,
    prng: Prng,
    limit: Limit,
    _i: PhantomData<I>,
    _o: PhantomData<O>,
}

impl<I, O, IC, OG> FnBuilder<I, O, IC, OG>
where
    IC: Cogen<I>,
    OG: GenOnce<O>,
{
    pub fn build_fn_once(self) -> impl FnOnce(I) -> O {
        move |input| {
            let randomness = self.input_cogen.cogen(input);
            let mut prng = self.prng;
            prng.reseed(randomness);
            let mut dice = Dice::new(&mut prng, self.limit);
            self.output_gen.gen_once(&mut dice)
        }
    }
}

impl<I, O, IC, OG> FnBuilder<I, O, IC, OG>
where
    IC: Cogen<I>,
    OG: Gen<O>,
{
    pub fn build_fn(self) -> impl Fn(I) -> O {
        move |input| {
            let randomness = self.input_cogen.cogen(input);
            let mut prng = self.prng.clone();
            prng.reseed(randomness);
            let mut dice = Dice::new(&mut prng, self.limit);
            self.output_gen.gen(&mut dice)
        }
    }

    pub fn build_fn_mut(mut self) -> impl FnMut(I) -> O {
        move |input| {
            let randomness = self.input_cogen.cogen(input);
            let prng = &mut self.prng;
            prng.reseed(randomness);
            let mut dice = Dice::new(prng, self.limit);
            self.output_gen.gen(&mut dice)
        }
    }
}

/// Generator for random functions. The generator returns a builder that can be converted into a
/// `Fn`, FnMut` or `FnOnce` implementation.
///
/// This generator does not directly generate an implementation of `Fn`, FnMut` or `FnOnce` because
/// implementing these traits is not stable yet.
pub fn fn_builder<I, O, IC, OG>(
    input_cogen: IC,
    output_gen: OG,
) -> impl GenOnce<FnBuilder<I, O, IC, OG>>
    where
        IC: Cogen<I>,
        OG: GenOnce<O>,
{
    gens::from_fn_once(|dice| {
        FnBuilder {
            input_cogen,
            output_gen,
            prng: dice.prng.fork(),
            limit: dice.limit(),
            _i: PhantomData,
            _o: PhantomData,
        }
    })
}