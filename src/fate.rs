use crate::{DieOnce, Limit, Prng};

/// Contains parameters for controlling the value generation with [`DieOnce`] and [`Die`].
///
/// The first parameter is a [`Prng`]. It is the only source of randomness that a implementor of
/// [`DieOnce`] or [`Die`] is allowed to use. Using the [`Prng`] will mutate its state, but for
/// the cause of preventing misuse there is no direct write access to it.
///
/// The second parameter is a [`Limit`]. It's the upper limit for the length of dynamic data
/// structures generated by the implementor of `DieOnce` or [`Die`]. The implementor has only read
/// access to the [`Limit`].
///
/// [`Die`]: crate::Die
pub struct Fate<'a> {
    prng: &'a mut Prng,
    limit: Limit,
}

impl<'a> Fate<'a> {
    /// Creates a new instance that uses the given parameters for value generation.
    pub fn new(prng: &'a mut Prng, limit: Limit) -> Self {
        Self { prng, limit }
    }

    /// Returns the next pseudorandom number generated with the underlying [`Prng`].
    pub fn next_number(&mut self) -> u64 {
        self.prng.next_number()
    }

    /// Returns a [`Prng`] split off from the underlying [`Prng`].
    pub fn fork_prng(&mut self) -> Prng {
        self.prng.fork()
    }

    /// Returns the underlying [`Limit`].
    pub fn limit(&self) -> Limit {
        self.limit
    }

    /// Creates a borrowed copy.
    ///
    /// [`Fate`] cannot implement the [`Copy`] trait because it contains a mutable
    /// reference. When it's necessary to move [`Fate`] multiple times this functions provides a
    /// convenient workaround.
    ///
    /// # Example
    ///
    /// ```
    /// use dicetest::{Limit, Fate, Prng};
    ///
    /// let mut prng = Prng::from_seed(42.into());
    /// let limit = Limit::default();
    /// let mut fate = Fate::new(&mut prng, limit);
    ///
    /// pub fn take_fate(_fate: Fate) {}
    ///
    /// take_fate(fate.copy());
    /// take_fate(fate);
    /// ```
    pub fn copy(&mut self) -> Fate {
        Fate {
            prng: self.prng,
            limit: self.limit,
        }
    }

    /// Creates a copy with the given limit.
    pub fn with_limit(&mut self, limit: Limit) -> Fate {
        let mut fate = self.copy();
        fate.limit = limit;
        fate
    }

    /// Generates a value with the given [`DieOnce`] using `self` as parameter.
    ///
    /// This function is more convenient than calling [`DieOnce::roll_once`] directly because
    /// it borrows the [`Fate`] instead of moving it.
    ///
    /// ```
    /// use dicetest::prelude::*;
    /// use dicetest::{Limit, Prng};
    ///
    /// let mut prng = Prng::from_seed(42.into());
    /// let limit = Limit::default();
    /// let mut fate = Fate::new(&mut prng, limit);
    ///
    /// let die = dice::bool();
    ///
    /// let val1 = fate.roll(&die); // Borrows `fate`
    /// let val2 = die.roll(fate); // Moves `fate`
    /// ```
    pub fn roll<T, D: DieOnce<T>>(&mut self, die: D) -> T {
        die.roll_once(self.copy())
    }

    /// Generates a value using the given [`Distribution`].
    ///
    /// Only available if the feature `rand` is enabled.
    ///
    /// [`Distribution`]: rand::distributions::Distribution
    #[cfg(feature = "rand")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
    pub fn roll_distribution<T, D>(&mut self, distribution: D) -> T
    where
        D: rand::distributions::Distribution<T>,
    {
        let die = crate::dice::from_distribution(distribution);
        self.roll(die)
    }
}

#[cfg(feature = "rand_core")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand_core")))]
impl<'a> rand_core::RngCore for Fate<'a> {
    fn next_u32(&mut self) -> u32 {
        self.next_number() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.next_number()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}
